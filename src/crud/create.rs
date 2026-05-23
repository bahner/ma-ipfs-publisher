//! `:create <name>` CRUD handler — create a new user namespace with an
//! owner-only gate ACL.
//!
//! The caller must hold `create` (or `*`) in the root transport ACL.
//! The created namespace receives a gate ACL that grants the caller `["*"]`.

use anyhow::{Context, Result};
use ciborium::Value as CborValue;
use ma_core::{AclMap, CapabilityEntry, CAP_CREATE};
use tracing::{info, warn};

use crate::entity::{IpldLink, NamespaceNode};

use super::helpers::{
    acl_cache_update, load_manifest, send_crud_i18n_error, send_crud_ok_cid, with_manifest_crud,
};
use super::CrudHandlerCtx;

/// Namespace names that may not be created by users.
const RESERVED_NS: &[&str] = &[
    "acl", "acls", "protocol", "kinds", "entities", "i18n", "config", "create",
];

/// Entry point for `crud-set` messages addressed to `:create`.
///
/// Expected payload: `[":create", "<name>"]`.
/// Any other form returns a usage error.
pub(super) async fn handle_create_ns(
    message: &ma_core::Message,
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (Some(""), [CborValue::Text(ns_name)]) => {
            let ns_name = ns_name.clone();
            create_namespace(message, &ns_name, reply_type, ctx).await
        }
        _ => send_crud_i18n_error(message, reply_type, ctx, "namespace-create-usage").await,
    }
}

async fn create_namespace(
    message: &ma_core::Message,
    ns_name: &str,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    // Capability check — caller must hold `create` (or wildcard `*`).
    {
        let acl = ctx.root_acl.read().await;
        if let Err(e) =
            crate::acl::check_full(&acl, &message.from, &[CAP_CREATE], |_| async { Ok(vec![]) })
                .await
        {
            warn!(
                from = %message.from,
                ns   = %ns_name,
                error = %e,
                "{}",
                crate::i18n::t("namespace-create-denied")
            );
            return send_crud_i18n_error(message, reply_type, ctx, "namespace-create-denied").await;
        }
    }

    // Reject reserved namespace names.
    if RESERVED_NS.contains(&ns_name) {
        return send_crud_i18n_error(message, reply_type, ctx, "namespace-name-reserved").await;
    }

    // Reject if the namespace already exists.
    {
        let manifest = load_manifest(ctx).await?;
        if manifest.namespaces.contains_key(ns_name) {
            return send_crud_i18n_error(message, reply_type, ctx, "namespace-already-exists")
                .await;
        }
    }

    // Build a creator-only gate ACL: caller gets all capabilities.
    let mut owner_acl = AclMap::new();
    owner_acl.insert(
        crate::acl::normalize_principal(&message.from).to_string(),
        CapabilityEntry::from_caps(["*"]),
    );

    // Store the ACL document in IPFS.
    let acl_cid = crate::kubo::dag_put(ctx.kubo_rpc_url, &owner_acl)
        .await
        .context("dag_put namespace gate ACL")?;

    // Assemble the NamespaceNode pointing at the new gate ACL.
    let ns_node = NamespaceNode {
        acl: Some(IpldLink::new(&acl_cid)),
        ..NamespaceNode::default()
    };

    // Persist the updated manifest.
    let ns_name_owned = ns_name.to_string();
    let new_root = with_manifest_crud(ctx, move |m| {
        m.namespaces.insert(ns_name_owned, ns_node);
        Ok(())
    })
    .await?;

    // Warm the ACL cache so blob operations on the new namespace work immediately.
    let cache_key = format!("{ns_name}.acl");
    acl_cache_update(ctx, &cache_key, &acl_cid).await;

    info!(
        ns    = %ns_name,
        owner = %message.from,
        cid   = %new_root,
        "{}",
        crate::i18n::t("namespace-created")
    );
    send_crud_ok_cid(message, reply_type, ctx, &new_root).await
}
