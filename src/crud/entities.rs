use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use tracing::info;

use crate::acl::check_full;
use crate::entity::{EntityNode, IpldLink};

use super::helpers::{
    load_manifest, register_entity_plugin, send_crud_i18n_error, send_crud_i18n_errorf,
    send_crud_ok, send_crud_ok_cid, send_crud_reply, send_crud_reply_cbor, with_manifest_crud,
};
use super::CrudHandlerCtx;

// ── Management capability helpers ─────────────────────────────────────────────

async fn check_entity_management_cap(
    message: &ma_core::Message,
    ctx: &CrudHandlerCtx<'_>,
    caps: &[&str],
) -> Result<()> {
    let acl = ctx.root_acl.read().await;
    check_full(&acl, &message.from, caps, |_| async { Ok(vec![]) })
        .await
        .with_context(|| {
            format!(
                "entity management denied for {}: requires {:?}",
                message.from, caps
            )
        })
}

// ── Entities namespace ─────────────────────────────────────────────────────────

pub(super) async fn handle_entities_ns(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    match rest.len() {
        0 => match (tail, args.as_slice()) {
            (None, []) => {
                info!("{}", crate::i18n::t("root-list-entities"));
                let names: Vec<String> = ctx.entity_registry.read().await.keys().cloned().collect();
                send_crud_reply_cbor(message, reply_type, ctx, &names).await
            }
            (Some(""), _) => {
                send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await
            }
            _ => Err(anyhow!("unknown entities operation")),
        },
        1 => handle_single_entity(message, &rest[0], tail, args, reply_type, ctx).await,
        2.. => {
            handle_entity_field(message, &rest[0], &rest[1..], tail, args, reply_type, ctx).await
        }
    }
}

#[allow(clippy::too_many_lines)]
async fn handle_single_entity(
    message: &ma_core::Message,
    name: &String,
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let manifest = load_manifest(ctx).await?;
            let link = manifest
                .entities
                .get(name.as_str())
                .ok_or_else(|| anyhow!("entity not found: {name}"))?;
            let entity: EntityNode = crate::kubo::dag_get(ctx.kubo_rpc_url, &link.cid).await?;
            let mut out = Vec::new();
            ciborium::ser::into_writer(&entity, &mut out)
                .context("serialising entity node as CBOR")?;
            send_crud_reply(message, reply_type, ctx, &out).await
        }
        (Some(""), []) => {
            // Delete entity — requires `delete` + `entities` in root ACL.
            check_entity_management_cap(message, ctx, &["delete", "entities"]).await?;
            let name = name.as_str();
            ctx.entity_registry.write().await.remove(name);
            let new_root = with_manifest_crud(ctx, |m| {
                m.entities.remove(name);
                Ok(())
            })
            .await?;
            info!(name = %name, cid = %new_root, "{}", crate::i18n::t("entity-deleted"));
            send_crud_ok(message, reply_type, ctx).await
        }
        (Some(""), [CborValue::Text(path)]) => {
            // Upsert entity — requires `create` + `entities` in root ACL.
            check_entity_management_cap(message, ctx, &["create", "entities"]).await?;
            let name = name.as_str();
            if name.chars().any(char::is_control) {
                return send_crud_i18n_error(message, reply_type, ctx, "entity-name-invalid").await;
            }
            if crate::entity::RESERVED_ENTITY_NAMES.contains(&name) {
                return send_crud_i18n_errorf(
                    message,
                    reply_type,
                    ctx,
                    "reserved-entity-name",
                    &[("name", name)],
                )
                .await;
            }
            let cid = crate::kubo::dag_resolve(ctx.kubo_rpc_url, path)
                .await
                .with_context(|| format!("resolving path {path}"))?;
            let cid = cid.as_str();
            let entity_node: EntityNode = crate::kubo::dag_get(ctx.kubo_rpc_url, cid)
                .await
                .with_context(|| format!("fetching entity node from {cid}"))?;
            let new_root = with_manifest_crud(ctx, |m| {
                m.entities.insert(name.to_string(), IpldLink::new(cid));
                Ok(())
            })
            .await?;
            register_entity_plugin(ctx, name, &entity_node).await;
            info!(name = %name, cid = %cid, "{}", crate::i18n::t("entity-created"));
            send_crud_ok_cid(message, reply_type, ctx, &new_root).await
        }
        _ => Err(anyhow!("unknown entities.{name} operation")),
    }
}

// ── Entity field helpers ───────────────────────────────────────────────────────

pub(super) async fn fetch_entity_node(ctx: &CrudHandlerCtx<'_>, name: &str) -> Result<EntityNode> {
    let manifest = load_manifest(ctx).await?;
    let link = manifest
        .entities
        .get(name)
        .ok_or_else(|| anyhow!("entity not found: {name}"))?;
    crate::kubo::dag_get(ctx.kubo_rpc_url, &link.cid)
        .await
        .with_context(|| format!("fetching entity {name} from {}", link.cid))
}

pub(super) async fn update_entity_node(
    ctx: &CrudHandlerCtx<'_>,
    name: &str,
    entity: &EntityNode,
) -> Result<String> {
    let entity_cid = crate::kubo::dag_put(ctx.kubo_rpc_url, entity)
        .await
        .with_context(|| format!("publishing updated entity {name}"))?;
    with_manifest_crud(ctx, |m| {
        m.entities
            .insert(name.to_string(), IpldLink::new(&entity_cid));
        Ok(())
    })
    .await?;
    Ok(entity_cid)
}

async fn handle_entity_field(
    message: &ma_core::Message,
    name: &String,
    field_path: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    let Some((field, sub_path)) = field_path.split_first() else {
        return Err(anyhow!("empty field path in entity.{name}"));
    };

    // Generic GET — works for any leaf field without field-specific code.
    if tail.is_none() && args.is_empty() && sub_path.is_empty() {
        let entity = fetch_entity_node(ctx, name).await?;
        let mut entity_cbor = Vec::new();
        ciborium::ser::into_writer(&entity, &mut entity_cbor)
            .context("serializing entity for field GET")?;
        let cbor_map: CborValue = ciborium::de::from_reader(entity_cbor.as_slice())
            .context("re-parsing entity CBOR map")?;
        if let CborValue::Map(entries) = cbor_map {
            if let Some((_, value)) = entries
                .into_iter()
                .find(|(k, _)| matches!(k, CborValue::Text(s) if s == field))
            {
                let mut out = Vec::new();
                ciborium::ser::into_writer(&value, &mut out)
                    .context("encoding field value as CBOR")?;
                return send_crud_reply(message, reply_type, ctx, &out).await;
            }
        }
        return Err(anyhow!("field '{field}' not found in entity '{name}'"));
    }

    match field.as_str() {
        "acl" => {
            handle_entity_acl_field(message, name, sub_path, tail, args, reply_type, ctx).await
        }
        _ => Err(anyhow!("unknown entity field '{field}' in entity.{name}")),
    }
}

async fn handle_entity_acl_field(
    message: &ma_core::Message,
    name: &String,
    sub_path: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    if !sub_path.is_empty() {
        return Err(anyhow!(
            "entity field 'acl' sub-path '{}' not yet implemented",
            sub_path.join(".")
        ));
    }
    match (tail, args.as_slice()) {
        (None, []) => {
            let entity = fetch_entity_node(ctx, name).await?;
            send_crud_reply_cbor(
                message,
                reply_type,
                ctx,
                &CborValue::Text(entity.acl.clone()),
            )
            .await
        }
        (Some(""), [CborValue::Text(acl_name)]) => {
            let manifest = load_manifest(ctx).await?;
            if !manifest.acls.contains_key(acl_name) {
                let available: Vec<&String> = manifest.acls.keys().collect();
                return Err(anyhow!(
                    "ACL name '{acl_name}' not found in manifest; available: {available:?}"
                ));
            }
            let mut entity = fetch_entity_node(ctx, name).await?;
            entity.acl = acl_name.clone();
            let entity_cid = update_entity_node(ctx, name, &entity).await?;
            info!(name = %name, acl_name = %acl_name, entity_cid = %entity_cid, "entity ACL name set");
            send_crud_ok_cid(message, reply_type, ctx, &entity_cid).await
        }
        (Some(""), []) => {
            let mut entity = fetch_entity_node(ctx, name).await?;
            entity.acl = String::new();
            let entity_cid = update_entity_node(ctx, name, &entity).await?;
            info!(name = %name, entity_cid = %entity_cid, "entity ACL cleared");
            send_crud_ok_cid(message, reply_type, ctx, &entity_cid).await
        }
        _ => Err(anyhow!("unknown entities.{name}.acl operation")),
    }
}
