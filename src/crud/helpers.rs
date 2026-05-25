use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use ma_core::{Did, DidDocumentResolver, IpfsGatewayResolver, Ipld, CONTENT_TYPE_TERM};
use tracing::{info, warn};

use crate::entity::{EntityNode, RuntimeManifest};

use super::CrudHandlerCtx;

// ── Path helpers ───────────────────────────────────────────────────────────────

/// Parse `:ns.seg1.seg2` → `("ns", ["seg1", "seg2"])`.
pub(super) fn parse_path(path: &str) -> Result<(&str, Vec<String>)> {
    let body = path
        .strip_prefix(':')
        .ok_or_else(|| anyhow!("CRUD path must start with ':' — got: {path}"))?;
    let (ns, rest_str) = body.split_once('.').unwrap_or((body, ""));
    if ns.is_empty() {
        return Err(anyhow!("CRUD path has no namespace: {path}"));
    }
    let segs: Vec<String> = rest_str
        .split('.')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    Ok((ns, segs))
}

/// Decoded CRUD operation from a single incoming message payload.
pub(super) enum CrudOp {
    /// `[":get", ":path"]`
    Get(String),
    /// `[":path", value]` — value is a CBOR scalar or IPFS path text
    Set(String, CborValue),
    /// `[":delete", ":path"]`
    Delete(String),
}

/// Decode a `application/x-ma-crud` payload.
///
/// Payload must be a two-element CBOR array:
/// - `[":get", ":path"]` → GET
/// - `[":delete", ":path"]` → DELETE
/// - `[":path", value]` → SET
pub(super) fn decode_crud_payload(content: &[u8]) -> Result<CrudOp> {
    let val: CborValue =
        ciborium::de::from_reader(content).context("invalid CBOR in CRUD payload")?;
    let CborValue::Array(items) = val else {
        return Err(anyhow!("CRUD payload must be a 2-element CBOR array"));
    };
    if items.len() != 2 {
        return Err(anyhow!(
            "CRUD payload must be a 2-element CBOR array, got {}",
            items.len()
        ));
    }
    let mut it = items.into_iter();
    let first = it.next().expect("len==2");
    let second = it.next().expect("len==2");
    match first {
        CborValue::Text(verb) if verb == ":get" => {
            let CborValue::Text(path) = second else {
                return Err(anyhow!("CRUD get: path must be a text string"));
            };
            Ok(CrudOp::Get(path))
        }
        CborValue::Text(verb) if verb == ":delete" => {
            let CborValue::Text(path) = second else {
                return Err(anyhow!("CRUD delete: path must be a text string"));
            };
            Ok(CrudOp::Delete(path))
        }
        CborValue::Text(path) => Ok(CrudOp::Set(path, second)),
        _ => Err(anyhow!(
            "CRUD payload: first element must be a text path or verb"
        )),
    }
}

/// Return `true` if `s` is a bare `CIDv1` (multibase base32-lowercase, prefix `b`).
///
/// `CIDv1` strings start with `b` (base32 lowercase multibase prefix) and are
/// self-describing via the embedded multicodec.  `CIDv0` (`Qm…`) is rejected.
pub(super) fn is_cidv1(s: &str) -> bool {
    s.starts_with('b') && s.len() > 10
}

// ── Manifest helpers ───────────────────────────────────────────────────────────

pub(super) async fn with_manifest_crud<F>(ctx: &CrudHandlerCtx<'_>, f: F) -> Result<String>
where
    F: FnOnce(&mut RuntimeManifest) -> Result<()>,
{
    let old_cid = current_root_cid(ctx).await?;
    let mut manifest: RuntimeManifest = crate::kubo::dag_get(ctx.kubo_rpc_url, &old_cid).await?;
    f(&mut manifest)?;
    let new_cid = crate::kubo::dag_put(ctx.kubo_rpc_url, &manifest).await?;
    if let Err(e) = crate::kubo::pin_update(ctx.kubo_rpc_url, &old_cid, &new_cid).await {
        warn!(old = %old_cid, new = %new_cid, error = %e, "CRUD pin_update failed");
    }
    update_stats_entities(ctx).await;
    ctx.stats.write().await.root_cid = Some(new_cid.clone());
    Ok(new_cid)
}

pub(super) async fn current_root_cid(ctx: &CrudHandlerCtx<'_>) -> Result<String> {
    ctx.stats
        .read()
        .await
        .root_cid
        .clone()
        .ok_or_else(|| anyhow!("no manifest root CID available"))
}

/// Fetch and deserialise the current `RuntimeManifest` from IPFS.
pub(super) async fn load_manifest(ctx: &CrudHandlerCtx<'_>) -> Result<RuntimeManifest> {
    let root_cid = current_root_cid(ctx).await?;
    crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await
}

/// Load an ACL from `cid`, insert it into `acl_cache` under `cache_key`.
/// Logs success or failure; non-fatal either way.
pub(super) async fn acl_cache_update(ctx: &CrudHandlerCtx<'_>, cache_key: &str, cid: &str) {
    match crate::acl::load_acl_from_cid(ctx.kubo_rpc_url, cid).await {
        Ok(acl_map) => {
            ctx.acl_cache
                .write()
                .await
                .insert(cache_key.to_string(), acl_map);
            info!(key = %cache_key, %cid, "ACL loaded into cache");
        }
        Err(e) => {
            warn!(key = %cache_key, %cid, error = %e, "failed to load ACL into cache");
        }
    }
}

/// Load a plugin from `entity_node` and register it in the entity registry.
/// Logs a warning if loading fails; non-fatal.
pub(super) async fn register_entity_plugin(
    ctx: &CrudHandlerCtx<'_>,
    name: &str,
    entity_node: &EntityNode,
) {
    match crate::plugin::EntityPlugin::load(name.to_string(), entity_node, ctx.kubo_rpc_url).await {
        Ok(ep) => {
            ctx.entity_registry
                .write()
                .await
                .insert(name.to_string(), Arc::new(ep));
        }
        Err(e) => warn!(
            name = %name,
            error = %e,
            "{}",
            crate::i18n::t("entity-load-failed")
        ),
    }
}

async fn update_stats_entities(ctx: &CrudHandlerCtx<'_>) {
    let names: Vec<String> = ctx.entity_registry.read().await.keys().cloned().collect();
    ctx.stats.write().await.entity_names = names;
}

// ── i18n helpers ───────────────────────────────────────────────────────────────

/// Resolve caller's DID document and extract their preferred language.
/// Falls back to the runtime's own language on any error.
pub(super) async fn caller_lang(from: &str, resolver: &IpfsGatewayResolver) -> String {
    if let Ok(doc) = resolver.resolve(from).await {
        if let Some(Ipld::Map(ma)) = &doc.ma {
            if let Some(Ipld::String(lang)) = ma.get("lang") {
                if crate::i18n::has_lang(lang) {
                    return lang.clone();
                }
            }
        }
    }
    crate::i18n::runtime_lang()
}

/// Send a CRUD error reply with a message localised to the caller's language.
pub(super) async fn send_crud_i18n_error(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
    key: &str,
) -> Result<()> {
    let lang = caller_lang(&incoming.from, ctx.resolver.as_ref()).await;
    send_crud_error(incoming, reply_type, ctx, &crate::i18n::t_lang(&lang, key)).await
}

/// Like [`send_crud_i18n_error`] but substitutes `%name%` placeholders in the
/// translated message before sending.
pub(super) async fn send_crud_i18n_errorf(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
    key: &str,
    args: &[(&str, &str)],
) -> Result<()> {
    let lang = caller_lang(&incoming.from, ctx.resolver.as_ref()).await;
    send_crud_error(
        incoming,
        reply_type,
        ctx,
        &crate::i18n::tf_lang(&lang, key, args),
    )
    .await
}

// ── Reply helpers ──────────────────────────────────────────────────────────────

pub(super) async fn send_crud_ok(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(&CborValue::Text(":ok".to_string()), &mut out)
        .context("encoding :ok")?;
    send_crud_reply(incoming, reply_type, ctx, &out).await
}

pub(super) async fn send_crud_ok_cid(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
    cid: &str,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Array(vec![
            CborValue::Text(":ok".to_string()),
            CborValue::Text(cid.to_string()),
        ]),
        &mut out,
    )
    .context("encoding [:ok, cid]")?;
    send_crud_reply(incoming, reply_type, ctx, &out).await
}

pub(super) async fn send_crud_error(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
    reason: &str,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Array(vec![
            CborValue::Text(":error".to_string()),
            CborValue::Text(reason.to_string()),
        ]),
        &mut out,
    )
    .context("encoding error reply")?;
    send_crud_reply(incoming, reply_type, ctx, &out).await
}

pub(super) async fn send_crud_reply_cbor<T: serde::Serialize + Sync>(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
    value: &T,
) -> Result<()> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(value, &mut out).context("encoding CBOR reply")?;
    send_crud_reply(incoming, reply_type, ctx, &out).await
}

pub(super) async fn send_crud_reply(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
    content: &[u8],
) -> Result<()> {
    send_crud_reply_raw(incoming, reply_type, ctx, CONTENT_TYPE_TERM, content).await
}

async fn send_crud_reply_raw(
    incoming: &ma_core::Message,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
    content_type: &str,
    content: &[u8],
) -> Result<()> {
    let sender = Did::try_from(incoming.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", incoming.from))?;

    let mut reply = ma_core::Message::new(
        ctx.our_did,
        &incoming.from,
        reply_type,
        content_type,
        content,
        ctx.signing_key,
    )
    .context("failed to build CRUD reply")?;
    reply.reply_to = Some(incoming.id.clone());

    match ctx
        .endpoint
        .outbox(
            ctx.resolver.as_ref(),
            &sender.base_id(),
            ma_core::CRUD_PROTOCOL_ID,
        )
        .await
    {
        Ok(mut outbox) => {
            outbox
                .send(&reply)
                .await
                .context("CRUD reply send failed")?;
            info!(to = %incoming.from, reply_to = %incoming.id, "CRUD reply sent");
        }
        Err(err) => {
            warn!(error = %err, to = %incoming.from, "CRUD reply delivery failed");
        }
    }
    Ok(())
}
