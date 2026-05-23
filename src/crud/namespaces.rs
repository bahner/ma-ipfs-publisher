use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use tracing::{debug, info};

use crate::acl::{AclCache, AclMap, OWNERS_PRINCIPAL};
use crate::entity::{IpldLink, NamespaceNode};

use super::helpers::{
    acl_cache_update, current_root_cid, load_manifest, send_crud_error, send_crud_i18n_error,
    send_crud_ok_cid, send_crud_reply_cbor, send_crud_reply_yaml, with_manifest_crud,
};
use super::CrudHandlerCtx;

// ── Namespace dispatching ─────────────────────────────────────────────────────

/// Namespace names that are reserved and may not be used as user namespaces.
const RESERVED_NS: &[&str] = &[
    "acl", "acls", "protocol", "kinds", "entities", "i18n", "config",
];

/// Check the namespace gate ACL for `caller` against `caps`.
async fn ns_acl_check(
    ns: &str,
    caller: &str,
    caps: &[&str],
    acl_cache: &AclCache,
    kubo_rpc_url: &str,
    root_cid: &str,
) -> Result<()> {
    let gate_key = format!("{ns}.acl");
    let maybe_acl = acl_cache.read().await.get(&gate_key).cloned();
    let acl = maybe_acl.ok_or_else(|| anyhow!("no gate ACL for namespace {ns}: access denied"))?;
    let url = kubo_rpc_url.to_string();
    let rc = root_cid.to_string();
    crate::acl::check_full(&acl, caller, caps, |g| {
        let url = url.clone();
        let rc = rc.clone();
        let g = g.to_string();
        async move { crate::acl::fetch_group_members(&url, &g, &rc).await }
    })
    .await
    .with_context(|| format!("namespace gate check failed for {caller} in {ns}"))
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn handle_namespace_op(
    message: &ma_core::Message,
    ns: &str,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    if ns.is_empty() {
        debug!("{}", crate::i18n::t("unknown-rpc-atom"));
        return Ok(());
    }
    let category = rest.first().map_or("", String::as_str);
    let sub_rest: &[String] = if rest.len() > 1 { &rest[1..] } else { &[] };

    // Gate check applies to blob operations only (not acl/acls management or ns root).
    if !matches!(category, "" | "acl" | "acls") {
        if let Ok(root_cid) = current_root_cid(ctx).await {
            let is_read = tail.is_none();
            let caps: &[&str] = if is_read {
                &[category, "read", "*"]
            } else {
                &[category, "update", "*"]
            };
            if let Err(e) = ns_acl_check(
                ns,
                &message.from,
                caps,
                &ctx.acl_cache,
                ctx.kubo_rpc_url,
                &root_cid,
            )
            .await
            {
                return send_crud_error(message, reply_type, ctx, &e.to_string()).await;
            }
        }
    }

    match category {
        "acl" => handle_ns_acl_gate(message, ns, tail, args, reply_type, ctx).await,
        "acls" => handle_ns_acls(message, ns, sub_rest, tail, args, reply_type, ctx).await,
        "" => handle_ns_root(message, ns, tail, args, reply_type, ctx).await,
        key => handle_ns_blob(message, ns, key, sub_rest, tail, args, reply_type, ctx).await,
    }
}

async fn handle_ns_root(
    message: &ma_core::Message,
    ns: &str,
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let manifest = load_manifest(ctx).await?;
            match manifest.namespaces.get(ns) {
                Some(ns_node) => send_crud_reply_cbor(message, reply_type, ctx, ns_node).await,
                None => send_crud_i18n_error(message, reply_type, ctx, "namespace-not-found").await,
            }
        }
        // Create / upsert namespace
        (Some(""), _) => {
            if RESERVED_NS.contains(&ns) {
                return send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await;
            }
            let new_root = with_manifest_crud(ctx, |m| {
                m.namespaces
                    .entry(ns.to_string())
                    .or_insert_with(NamespaceNode::default);
                Ok(())
            })
            .await?;
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        _ => Err(anyhow!("unknown namespace '{ns}' operation")),
    }
}

async fn handle_ns_acl_gate(
    message: &ma_core::Message,
    ns: &str,
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let manifest = load_manifest(ctx).await?;
            let ns_node = manifest
                .namespaces
                .get(ns)
                .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
            match &ns_node.acl {
                Some(link) => {
                    send_crud_reply_cbor(
                        message,
                        reply_type,
                        ctx,
                        &CborValue::Text(link.cid.clone()),
                    )
                    .await
                }
                None => send_crud_i18n_error(message, reply_type, ctx, "no-ns-gate-acl").await,
            }
        }
        (Some(""), [CborValue::Text(cid)]) => {
            let cid = cid.clone();
            let new_root = with_manifest_crud(ctx, |m| {
                let ns_node = m
                    .namespaces
                    .get_mut(ns)
                    .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                ns_node.acl = Some(IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            let cache_key = format!("{ns}.acl");
            acl_cache_update(ctx, &cache_key, &cid).await;
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        (Some(""), []) => {
            let new_root = with_manifest_crud(ctx, |m| {
                let ns_node = m
                    .namespaces
                    .get_mut(ns)
                    .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                ns_node.acl = None;
                Ok(())
            })
            .await?;
            let cache_key = format!("{ns}.acl");
            ctx.acl_cache.write().await.remove(&cache_key);
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        _ => Err(anyhow!("unknown {ns}.acl operation")),
    }
}

async fn handle_ns_acls(
    message: &ma_core::Message,
    ns: &str,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    match (rest, tail, args.as_slice()) {
        ([], None, []) => {
            let manifest = load_manifest(ctx).await?;
            let ns_node = manifest
                .namespaces
                .get(ns)
                .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
            let names: Vec<CborValue> = ns_node
                .acls
                .keys()
                .map(|k| CborValue::Text(k.clone()))
                .collect();
            send_crud_reply_cbor(
                message,
                reply_type,
                ctx,
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Array(names),
                ]),
            )
            .await
        }
        ([acl_name], None, []) => {
            let manifest = load_manifest(ctx).await?;
            let ns_node = manifest
                .namespaces
                .get(ns)
                .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
            let link = ns_node
                .acls
                .get(acl_name.as_str())
                .ok_or_else(|| anyhow!("ACL not found: {ns}.acls.{acl_name}"))?;
            send_crud_reply_cbor(message, reply_type, ctx, &CborValue::Text(link.cid.clone())).await
        }
        ([acl_name], Some(""), [CborValue::Text(cid)]) => {
            let acl_name = acl_name.clone();
            let cid = cid.clone();
            let new_root = with_manifest_crud(ctx, |m| {
                let ns_node = m
                    .namespaces
                    .get_mut(ns)
                    .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                ns_node.acls.insert(acl_name.clone(), IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            let cache_key = format!("{ns}.acls.{acl_name}");
            acl_cache_update(ctx, &cache_key, &cid).await;
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        ([acl_name], Some(""), []) => {
            let acl_name = acl_name.clone();
            let new_root = with_manifest_crud(ctx, |m| {
                let ns_node = m
                    .namespaces
                    .get_mut(ns)
                    .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                ns_node.acls.remove(&acl_name);
                Ok(())
            })
            .await?;
            let cache_key = format!("{ns}.acls.{acl_name}");
            ctx.acl_cache.write().await.remove(&cache_key);
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        _ => Err(anyhow!("unknown {ns}.acls operation")),
    }
}

#[allow(clippy::too_many_arguments)]
async fn handle_ns_blob(
    message: &ma_core::Message,
    ns: &str,
    key: &str,
    sub_path: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let manifest = load_manifest(ctx).await?;
            let ns_node = manifest
                .namespaces
                .get(ns)
                .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
            let link_cid = ns_node
                .extra
                .get(key)
                .and_then(|v| v.get("/"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("key not found: {ns}.{key}"))?;
            let resolved_cid = if sub_path.is_empty() {
                link_cid.to_string()
            } else {
                let ipfs_path = format!("/ipfs/{}/{}", link_cid, sub_path.join("/"));
                crate::kubo::dag_resolve(ctx.kubo_rpc_url, &ipfs_path)
                    .await
                    .with_context(|| format!("traversing {ns}.{key}.{}", sub_path.join(".")))?
            };
            send_crud_reply_cbor(message, reply_type, ctx, &CborValue::Text(resolved_cid)).await
        }
        (Some(""), [CborValue::Text(cid)]) if sub_path.is_empty() => {
            let cid = cid.clone();
            let new_root = with_manifest_crud(ctx, |m| {
                let ns_node = m
                    .namespaces
                    .get_mut(ns)
                    .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                ns_node
                    .extra
                    .insert(key.to_string(), serde_json::json!({ "/": cid }));
                Ok(())
            })
            .await?;
            info!(ns = %ns, key = %key, cid = %cid, "blob registered");
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        (Some(""), []) if sub_path.is_empty() => {
            let new_root = with_manifest_crud(ctx, |m| {
                let ns_node = m
                    .namespaces
                    .get_mut(ns)
                    .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                ns_node.extra.remove(key);
                Ok(())
            })
            .await?;
            info!(ns = %ns, key = %key, "blob deleted");
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        _ => Err(anyhow!("unknown {ns}.{key} operation")),
    }
}

// ── Root transport-gate ACL ────────────────────────────────────────────────────

pub(super) async fn handle_root_acl(
    message: &ma_core::Message,
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None | Some("edit"), []) => {
            let manifest = load_manifest(ctx).await?;
            match &manifest.acl {
                Some(link) => {
                    let acl_map: AclMap =
                        crate::acl::load_acl_from_cid(ctx.kubo_rpc_url, &link.cid).await?;
                    let yaml =
                        serde_yaml::to_string(&acl_map).context("serialising root ACL as YAML")?;
                    send_crud_reply_yaml(message, reply_type, ctx, &yaml).await
                }
                None => send_crud_i18n_error(message, reply_type, ctx, "no-root-acl").await,
            }
        }
        (Some(""), [CborValue::Text(cid)]) => {
            let cid = cid.clone();
            let acl_map = crate::acl::load_acl_from_cid(ctx.kubo_rpc_url, &cid)
                .await
                .with_context(|| format!("loading root ACL from {cid}"))?;
            if !acl_map.contains_key(OWNERS_PRINCIPAL) {
                return send_crud_i18n_error(message, reply_type, ctx, "acl-missing-owners").await;
            }
            let new_root = with_manifest_crud(ctx, |m| {
                m.acl = Some(IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            *ctx.root_acl.write().await = acl_map;
            info!(cid = %cid, "Root transport ACL updated");
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        (Some("edit"), [CborValue::Bytes(dag_cbor)]) => {
            let dag_cbor = dag_cbor.clone();
            let cid = crate::kubo::dag_put_raw(ctx.kubo_rpc_url, &dag_cbor)
                .await
                .context("dag_put_raw for root ACL")?;
            let acl_map: AclMap = crate::kubo::dag_get(ctx.kubo_rpc_url, &cid)
                .await
                .with_context(|| format!("loading updated root ACL from {cid}"))?;
            if !acl_map.contains_key(OWNERS_PRINCIPAL) {
                return send_crud_i18n_error(message, reply_type, ctx, "acl-missing-owners").await;
            }
            let new_root = with_manifest_crud(ctx, |m| {
                m.acl = Some(IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            *ctx.root_acl.write().await = acl_map;
            info!(cid = %cid, "Root transport ACL updated via edit-save");
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        // DELETE is refused — the root transport ACL must never be cleared.
        (Some(""), []) => {
            send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await
        }
        _ => Err(anyhow!("unknown :acl operation")),
    }
}
