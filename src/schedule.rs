//! Scheduled entity dispatch — cron, one-shot (`at`), and random re-scheduling.
//!
//! ## Schedule types
//!
//! | Variant | Spec format | Behaviour |
//! |---------|------------|-----------|
//! | `Cron` | 6-field cron `"sec min hour day month weekday"` or English spec | Fires on schedule indefinitely. |
//! | `Interval` | Human-readable duration (`"1h"`, `"30m"`, `"5s"`, `"2h30m"`) | Fires every N seconds indefinitely. |
//! | `At` | Unix timestamp in milliseconds | Fires once after the computed delay. |
//! | `Random` | `random_max_secs: <u64>` | Fires after a random 1–N second delay, then self-reschedules. |
//!
//! ## ACL
//!
//! Scheduled dispatch bypasses all ACL checks.  The runtime is the trusted
//! caller; schedules are authorised by the entity definition or bootstrap
//! configuration that declares them.
//!
//! ## Outbound messages
//!
//! Outbound envelopes produced by a scheduled call (via `ma_send`) are
//! silently dropped — there is no external sender to route replies back to.
//! State changes via `ma_set_state` are persisted to IPFS normally.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use ciborium::Value as CborValue;
use ma_core::{ipfs_add, CONTENT_TYPE_TERM};
use serde::{Deserialize, Serialize};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::warn;

use crate::entity::{CastInput, LocalMessage, PluginCtx, PluginKind};
use crate::plugin::EntityRegistry;

// ── Static schedule (YAML entity definition) ──────────────────────────────────

/// One schedule entry declared statically in an [`crate::entity::EntityNode`].
///
/// Exactly one of `cron`, `interval`, or `random_max_secs` must be set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSchedule {
    /// 6-field cron spec or English spec (e.g. `"0 0 * * * *"`, `"every hour"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    /// Fixed repeat interval as a human-readable duration (e.g. `"1h"`, `"30m"`, `"5s"`).
    /// Supports `s`, `m`, `h`, `d` units, combinable: `"1h30m"`, `"2d12h"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Random re-scheduling: fire after 1–N seconds, then add a new random job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_max_secs: Option<u64>,
    /// CBOR verb atom to invoke, e.g. `":chime"`.
    pub verb: String,
    /// Optional positional arguments.  Converted to CBOR when the job fires.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<serde_yaml::Value>,
}

// ── Schedule request ──────────────────────────────────────────────────────────

/// A schedule request, either produced from a static entity definition or
/// enqueued dynamically by a plugin via host functions.
#[derive(Debug, Clone)]
pub enum ScheduleRequest {
    /// Recurring cron / English schedule.
    Cron {
        spec: String,
        /// Pre-encoded CBOR call bytes (verb atom or `[":verb", arg…]` array).
        content: Vec<u8>,
    },
    /// Fixed-interval recurring dispatch.
    Interval { secs: u64, content: Vec<u8> },
    /// One-shot dispatch at a Unix millisecond timestamp.
    At { timestamp_ms: i64, content: Vec<u8> },
    /// Self-rescheduling one-shot with a random delay up to `max_secs`.
    Random { max_secs: u64, content: Vec<u8> },
}

impl ScheduleRequest {
    /// Build a [`ScheduleRequest`] from a [`StaticSchedule`] entry.
    ///
    /// Returns an error when no schedule type field is set or `interval` is
    /// unparsable.
    pub fn from_static(s: &StaticSchedule) -> Result<Self> {
        let content = encode_cbor_call(&s.verb, &s.args);
        if let Some(max_secs) = s.random_max_secs {
            return Ok(Self::Random { max_secs, content });
        }
        if let Some(ref dur_str) = s.interval {
            let dur = parse_duration(dur_str)?;
            return Ok(Self::Interval {
                secs: dur.as_secs(),
                content,
            });
        }
        if let Some(ref spec) = s.cron {
            return Ok(Self::Cron {
                spec: spec.clone(),
                content,
            });
        }
        bail!("schedule entry has no `cron`, `interval`, or `random_max_secs` field")
    }
}

// ── Scheduler context ─────────────────────────────────────────────────────────

/// Minimal context cloned into every scheduled job closure.
#[derive(Clone)]
pub struct SchedulerCtx {
    pub entity_registry: EntityRegistry,
    pub kubo_rpc_url: String,
    pub our_did: String,
}

// ── Job registration ──────────────────────────────────────────────────────────

/// Register a [`ScheduleRequest`] on the scheduler for the named entity.
///
/// Returns the scheduler-assigned [`uuid::Uuid`] for the created job.
pub async fn register_schedule(
    sched: &JobScheduler,
    ctx: SchedulerCtx,
    fragment: String,
    req: ScheduleRequest,
) -> Result<uuid::Uuid> {
    let id = match req {
        ScheduleRequest::Cron { spec, content } => {
            let job = Job::new_async(spec.as_str(), {
                let ctx = ctx.clone();
                let fragment = fragment.clone();
                move |_, _| {
                    let ctx = ctx.clone();
                    let fragment = fragment.clone();
                    let content = content.clone();
                    Box::pin(async move {
                        dispatch_scheduled(&ctx, &fragment, &content).await;
                    })
                }
            })?;
            sched.add(job).await?
        }

        ScheduleRequest::Interval { secs, content } => {
            let job = Job::new_repeated_async(Duration::from_secs(secs), {
                let ctx = ctx.clone();
                let fragment = fragment.clone();
                move |_, _| {
                    let ctx = ctx.clone();
                    let fragment = fragment.clone();
                    let content = content.clone();
                    Box::pin(async move {
                        dispatch_scheduled(&ctx, &fragment, &content).await;
                    })
                }
            })?;
            sched.add(job).await?
        }

        ScheduleRequest::At {
            timestamp_ms,
            content,
        } => {
            let now_ms = i64::try_from(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis(),
            )
            .unwrap_or(i64::MAX);
            let delay_ms = u64::try_from((timestamp_ms - now_ms).max(0)).unwrap_or(0);
            let job = Job::new_one_shot_async(Duration::from_millis(delay_ms), {
                let ctx = ctx.clone();
                let fragment = fragment.clone();
                move |_, _| {
                    let ctx = ctx.clone();
                    let fragment = fragment.clone();
                    let content = content.clone();
                    Box::pin(async move {
                        dispatch_scheduled(&ctx, &fragment, &content).await;
                    })
                }
            })?;
            sched.add(job).await?
        }

        ScheduleRequest::Random { max_secs, content } => {
            let job = make_random_job(sched.clone(), ctx, fragment, content, max_secs)?;
            sched.add(job).await?
        }
    };
    Ok(id)
}

/// Create a self-rescheduling one-shot random job.
///
/// After firing, it adds a new random job to the same scheduler so the
/// entity is called again after another random delay.  This is a named
/// function (not a closure) so the recursive call compiles without infinite
/// type regress.
pub fn make_random_job(
    sched: JobScheduler,
    ctx: SchedulerCtx,
    fragment: String,
    content: Vec<u8>,
    max_secs: u64,
) -> Result<Job> {
    let delay = rand_delay(max_secs);
    Ok(Job::new_one_shot_async(delay, move |_, _| {
        let sched = sched.clone();
        let ctx = ctx.clone();
        let fragment = fragment.clone();
        let content = content.clone();
        Box::pin(async move {
            dispatch_scheduled(&ctx, &fragment, &content).await;
            // Re-schedule with a new independent random delay.
            match make_random_job(sched.clone(), ctx, fragment.clone(), content, max_secs) {
                Ok(next) => {
                    if let Err(e) = sched.add(next).await {
                        warn!(fragment = %fragment, error = %e, "failed to reschedule random job");
                    }
                }
                Err(e) => {
                    warn!(fragment = %fragment, error = %e, "failed to create next random job");
                }
            }
        })
    })?)
}

fn rand_delay(max_secs: u64) -> Duration {
    use rand::Rng;
    let secs = rand::thread_rng().gen_range(1..=max_secs.max(1));
    Duration::from_secs(secs)
}

/// Parse a human-readable duration string into a [`Duration`].
///
/// Supported units: `s` (seconds), `m` (minutes), `h` (hours), `d` (days).
/// Units may be combined: `"1h30m"`, `"90s"`, `"2d12h"`.
pub fn parse_duration(s: &str) -> Result<Duration> {
    let mut total_secs: u64 = 0;
    let mut num_buf = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            num_buf.push(ch);
        } else {
            let n: u64 = num_buf
                .parse()
                .with_context(|| format!("invalid number in duration {s:?}"))?;
            num_buf.clear();
            let unit_secs = match ch {
                's' => n,
                'm' => n * 60,
                'h' => n * 3_600,
                'd' => n * 86_400,
                _ => bail!("unknown duration unit {ch:?} in {s:?}"),
            };
            total_secs += unit_secs;
        }
    }
    if !num_buf.is_empty() {
        bail!("duration {s:?} ends with a number but no unit");
    }
    if total_secs == 0 {
        bail!("duration must be > 0");
    }
    Ok(Duration::from_secs(total_secs))
}

// ── Scheduled dispatch ────────────────────────────────────────────────────────

/// Call an entity plugin on schedule, bypassing all ACL checks.
///
/// - Outbound envelopes from the call are silently dropped.
/// - State changes via `ma_set_state` are persisted to IPFS.
pub async fn dispatch_scheduled(ctx: &SchedulerCtx, fragment: &str, content: &[u8]) {
    let plugin = ctx.entity_registry.read().await.get(fragment).cloned();
    let Some(plugin) = plugin else {
        warn!(fragment = %fragment, "scheduled dispatch: entity not found");
        return;
    };

    let now_nanos = u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
    )
    .unwrap_or(u64::MAX);

    // Produce a stable message ID from the timestamp + fragment without
    // pulling in a uuid dep here — blake3 is already available.
    let mut hasher = blake3::Hasher::new();
    hasher.update(&now_nanos.to_le_bytes());
    hasher.update(fragment.as_bytes());
    let id = hasher.finalize().to_hex()[..16].to_string();

    let local_msg = LocalMessage {
        id,
        from: format!("{}#scheduler", ctx.our_did),
        to: format!("{}#{}", ctx.our_did, fragment),
        created_at: now_nanos,
        expires: 0,
        reply_to: None,
        content_type: CONTENT_TYPE_TERM.to_string(),
        content: content.to_vec(),
    };

    let cast_input = CastInput {
        msg: local_msg,
        ctx: PluginCtx {
            self_did: format!("{}#{}", ctx.our_did, fragment),
        },
    };

    let result = match plugin.kind {
        PluginKind::Stateless => plugin.handle_cast(&cast_input),
        PluginKind::Stateful => plugin.handle_call(&cast_input),
    };

    let result = match result {
        Ok(r) => r,
        Err(e) => {
            warn!(fragment = %fragment, error = %e, "scheduled dispatch error");
            return;
        }
    };

    // Outbound envelopes are silently dropped — no external sender to route to.
    let _ = result.envelopes;

    // Persist state if changed.
    if let Some(state_bytes) = result.pending_state {
        match ipfs_add(&ctx.kubo_rpc_url, state_bytes.clone()).await {
            Ok(_) => plugin.mark_saved(state_bytes),
            Err(e) => warn!(
                fragment = %fragment,
                error = %e,
                "scheduled dispatch: state save failed"
            ),
        }
    }
}

// ── CBOR call encoding ────────────────────────────────────────────────────────

/// Encode a verb + YAML args as pre-built CBOR call bytes.
///
/// No args → CBOR text atom `":verb"`.
/// With args → CBOR array `[":verb", arg1, …]`.
pub fn encode_cbor_call(verb: &str, args: &[serde_yaml::Value]) -> Vec<u8> {
    let mut out = Vec::new();
    if args.is_empty() {
        ciborium::ser::into_writer(&CborValue::Text(verb.to_string()), &mut out).ok();
    } else {
        let items: Vec<CborValue> = std::iter::once(CborValue::Text(verb.to_string()))
            .chain(args.iter().map(yaml_to_cbor))
            .collect();
        ciborium::ser::into_writer(&CborValue::Array(items), &mut out).ok();
    }
    out
}

/// Encode a verb + CBOR args as pre-built CBOR call bytes (host-function path).
///
/// No args → CBOR text atom `":verb"`.
/// With args → CBOR array `[":verb", arg1, …]`.
pub fn encode_cbor_call_cbor(verb: &str, args: &[CborValue]) -> Vec<u8> {
    let mut out = Vec::new();
    if args.is_empty() {
        ciborium::ser::into_writer(&CborValue::Text(verb.to_string()), &mut out).ok();
    } else {
        let mut items = vec![CborValue::Text(verb.to_string())];
        items.extend_from_slice(args);
        ciborium::ser::into_writer(&CborValue::Array(items), &mut out).ok();
    }
    out
}

fn yaml_to_cbor(v: &serde_yaml::Value) -> CborValue {
    match v {
        serde_yaml::Value::String(s) => CborValue::Text(s.clone()),
        serde_yaml::Value::Number(n) => n.as_i64().map_or_else(
            || n.as_f64().map_or(CborValue::Null, CborValue::Float),
            |i| CborValue::Integer(i.into()),
        ),
        serde_yaml::Value::Bool(b) => CborValue::Bool(*b),
        serde_yaml::Value::Null | serde_yaml::Value::Tagged(_) => CborValue::Null,
        serde_yaml::Value::Sequence(seq) => {
            CborValue::Array(seq.iter().map(yaml_to_cbor).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let pairs = map
                .iter()
                .map(|(k, v)| (yaml_to_cbor(k), yaml_to_cbor(v)))
                .collect();
            CborValue::Map(pairs)
        }
    }
}
