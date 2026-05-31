use anyhow::{anyhow, Result};
use ciborium::Value as CborValue;
use tracing::warn;

use super::helpers::{
    load_manifest, send_crud_i18n_error, send_crud_i18n_errorf, send_crud_ok, send_crud_ok_cid,
    send_crud_reply_cbor, with_manifest_crud,
};
use super::CrudHandlerCtx;

// ── Config key tables ──────────────────────────────────────────────────────────

/// Daemon config fields that may be read/written via CRUD and are saved to
/// `config.yaml` on change.
pub const DAEMON_CONFIG_KEYS_PUB: &[&str] = &[
    "kubo_rpc_url",
    "kubo_key_alias",
    "log_level",
    "log_level_stdout",
    "did_resolver_positive_ttl_secs",
    "did_resolver_negative_ttl_secs",
    "log_file",
];

const DAEMON_CONFIG_KEYS: &[&str] = DAEMON_CONFIG_KEYS_PUB;

/// Manifest config keys that may be written via CRUD (stored in IPFS DAG).
const MANIFEST_CONFIG_KEYS: &[&str] = &[
    "owners",
    "i18n",
    "did_document_publishing_interval_secs",
    "did_document_publishing_timeout_secs",
    "did_document_publishing_lifetime_hours",
    "ipns_publish_lifetime_hours",
    "ipns_publish_resolve",
    "ipns_publish_allow_offline",
    "status_cors_allowed_origins",
];

/// Keys that are never exposed or writable via CRUD.
/// Any key beginning with `secret` is also blocked dynamically.
const PROTECTED_CONFIG_KEYS: &[&str] = &[
    "slug",
    "secret_bundle",
    "secret_bundle_passphrase",
    "config_path",
];

pub fn is_protected_config_key_pub(key: &str) -> bool {
    PROTECTED_CONFIG_KEYS.contains(&key) || key.starts_with("secret")
}

fn is_protected_config_key(key: &str) -> bool {
    is_protected_config_key_pub(key)
}

/// Read a daemon config field as a `serde_yaml::Value` for CRUD responses.
/// Returns `Value::Null` for unknown or platform-guarded keys.
pub fn daemon_config_key_value_pub(cfg: &ma_core::Config, key: &str) -> serde_yaml::Value {
    match key {
        "kubo_rpc_url" => serde_yaml::Value::String(cfg.kubo_rpc_url.clone()),
        "kubo_key_alias" => serde_yaml::Value::String(cfg.kubo_key_alias.clone()),
        "log_level" => serde_yaml::Value::String(cfg.log_level.clone()),
        "log_level_stdout" => serde_yaml::Value::String(cfg.log_level_stdout.clone()),
        "did_resolver_positive_ttl_secs" => {
            serde_yaml::Value::Number(cfg.did_resolver_positive_ttl_secs.into())
        }
        "did_resolver_negative_ttl_secs" => {
            serde_yaml::Value::Number(cfg.did_resolver_negative_ttl_secs.into())
        }
        "log_file" => cfg.log_file.as_ref().map_or(serde_yaml::Value::Null, |p| {
            serde_yaml::Value::String(p.to_string_lossy().into_owned())
        }),
        _ => serde_yaml::Value::Null,
    }
}

/// Apply a YAML value from CRUD to the corresponding `Config` field in memory.
pub fn set_daemon_config_key_pub(cfg: &mut ma_core::Config, key: &str, val: &serde_yaml::Value) {
    match key {
        "kubo_rpc_url" => {
            if let Some(s) = val.as_str() {
                cfg.kubo_rpc_url = s.to_string();
            }
        }
        "kubo_key_alias" => {
            if let Some(s) = val.as_str() {
                cfg.kubo_key_alias = s.to_string();
            }
        }
        "log_level" => {
            if let Some(s) = val.as_str() {
                cfg.log_level = s.to_string();
            }
        }
        "log_level_stdout" => {
            if let Some(s) = val.as_str() {
                cfg.log_level_stdout = s.to_string();
            }
        }
        "did_resolver_positive_ttl_secs" => {
            if let Some(n) = val.as_u64() {
                cfg.did_resolver_positive_ttl_secs = n;
            }
        }
        "did_resolver_negative_ttl_secs" => {
            if let Some(n) = val.as_u64() {
                cfg.did_resolver_negative_ttl_secs = n;
            }
        }
        "log_file" => {
            cfg.log_file = val.as_str().map(std::path::PathBuf::from);
        }
        _ => {}
    }
}

fn set_daemon_config_key(cfg: &mut ma_core::Config, key: &str, val: &serde_yaml::Value) {
    set_daemon_config_key_pub(cfg, key, val);
}

/// Convert a CBOR value to a `serde_yaml::Value` for storage in
/// `RuntimeManifest.config`. Clients send native CBOR — text, integer,
/// boolean, float, null, arrays, maps — and this maps it to the YAML
/// value type that the config tree uses internally.
fn cbor_to_yaml(val: &CborValue) -> serde_yaml::Value {
    match val {
        CborValue::Bool(b) => serde_yaml::Value::Bool(*b),
        CborValue::Integer(i) => u64::try_from(*i).map_or_else(
            |_| {
                i64::try_from(*i).map_or(serde_yaml::Value::Null, |n| {
                    serde_yaml::Value::Number(n.into())
                })
            },
            |n| serde_yaml::Value::Number(n.into()),
        ),
        CborValue::Float(f) => serde_yaml::Value::Number((*f).into()),
        CborValue::Text(s) => serde_yaml::Value::String(s.clone()),
        CborValue::Bytes(b) => {
            serde_yaml::Value::String(b.iter().fold(String::new(), |mut acc, byte| {
                use std::fmt::Write;
                let _ = write!(acc, "{byte:02x}");
                acc
            }))
        }
        CborValue::Array(arr) => {
            serde_yaml::Value::Sequence(arr.iter().map(cbor_to_yaml).collect())
        }
        CborValue::Map(pairs) => {
            let mut map = serde_yaml::Mapping::new();
            for (k, v) in pairs {
                if let CborValue::Text(key) = k {
                    map.insert(serde_yaml::Value::String(key.clone()), cbor_to_yaml(v));
                }
            }
            serde_yaml::Value::Mapping(map)
        }
        CborValue::Tag(_, inner) => cbor_to_yaml(inner),
        _ => serde_yaml::Value::Null,
    }
}

// ── Config namespace ───────────────────────────────────────────────────────────

#[allow(clippy::too_many_lines)]
pub(super) async fn handle_config_ns(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    // No key segment — operate on config root.
    if rest.is_empty() {
        return match (tail, args.as_slice()) {
            (None, []) => {
                let manifest = load_manifest(ctx).await?;
                let mut combined = manifest.config.clone();
                {
                    let cfg = ctx.shared_config.read().await;
                    for key in DAEMON_CONFIG_KEYS {
                        let val = daemon_config_key_value_pub(&cfg, key);
                        if !val.is_null() {
                            combined.insert(key.to_string(), val);
                        }
                    }
                    drop(cfg);
                }
                send_crud_reply_cbor(message, reply_type, ctx, &combined).await
            }
            (Some(""), _) => {
                send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await
            }
            _ => Err(anyhow!("unknown config operation")),
        };
    }

    let [key] = rest else {
        return Err(anyhow!("unknown config operation"));
    };

    if is_protected_config_key(key.as_str()) {
        return send_crud_i18n_errorf(
            message,
            reply_type,
            ctx,
            "config-key-protected",
            &[("key", key.as_str())],
        )
        .await;
    }

    let is_daemon_key = DAEMON_CONFIG_KEYS.contains(&key.as_str());
    match (tail, args.as_slice()) {
        (None, []) => {
            let val = if is_daemon_key {
                let cfg = ctx.shared_config.read().await;
                daemon_config_key_value_pub(&cfg, key.as_str())
            } else {
                let manifest = load_manifest(ctx).await?;
                manifest
                    .config
                    .get(key.as_str())
                    .ok_or_else(|| anyhow!("config key not found: {key}"))?
                    .clone()
            };
            send_crud_reply_cbor(message, reply_type, ctx, &val).await
        }
        (Some(""), []) => {
            if is_daemon_key {
                return send_crud_i18n_errorf(
                    message,
                    reply_type,
                    ctx,
                    "config-key-no-delete",
                    &[("key", key.as_str())],
                )
                .await;
            }
            let key = key.as_str().to_string();
            with_manifest_crud(ctx, |m| {
                m.config.remove(&key);
                Ok(())
            })
            .await?;
            send_crud_ok(message, reply_type, ctx).await
        }
        (Some(""), [value]) => {
            let key = key.as_str().to_string();
            let yaml_val = cbor_to_yaml(value);
            if is_daemon_key {
                set_daemon_config_key(&mut *ctx.shared_config.write().await, &key, &yaml_val);
                let save_result = ctx.shared_config.read().await.save();
                if let Err(e) = save_result {
                    warn!(key = %key, error = %e, "failed to save config.yaml after CRUD update");
                }
                return send_crud_ok(message, reply_type, ctx).await;
            }
            // Manifest config key — only known keys may be written.
            if !MANIFEST_CONFIG_KEYS.contains(&key.as_str()) {
                return send_crud_i18n_errorf(
                    message,
                    reply_type,
                    ctx,
                    "config-key-not-manifest",
                    &[("key", key.as_str())],
                )
                .await;
            }
            let new_root = with_manifest_crud(ctx, |m| {
                m.config.insert(key.clone(), yaml_val.clone());
                Ok(())
            })
            .await?;
            // Language hot-swap: reload FTL messages immediately.
            if key == "i18n" {
                if let serde_yaml::Value::String(ref lang) = yaml_val {
                    crate::i18n::switch_lang(lang, ctx.kubo_rpc_url).await;
                }
            }
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        _ => Err(anyhow!("unknown config.{key} operation")),
    }
}
