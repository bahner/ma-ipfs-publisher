use anyhow::{anyhow, Result};
use ciborium::Value as CborValue;

use crate::entity::{IpldLink, KindNode};

use super::helpers::{
    is_cidv1, load_manifest, send_crud_i18n_error, send_crud_ok, send_crud_ok_cid,
    send_crud_reply_cbor, with_manifest_crud,
};
use super::CrudHandlerCtx;

// ── Kinds namespace ────────────────────────────────────────────────────────────

/// Handle `:kinds` CRUD operations.
///
/// Protocol IDs contain slashes and cannot appear as dot-path segments, so
/// individual kind mutations pass the protocol ID as part of the CBOR value:
///
/// | Operation | CRUD payload                                          |
/// |-----------|------------------------------------------------------|
/// | List all  | `[":get", ":kinds"]`                                 |
/// | Get kind  | `[":kinds", protocol_text]`                          |
/// | Upsert    | `[":kinds", [protocol_text, cid_text]]`               |
/// | Delete    | `[":kinds", [protocol_text]]`                        |
/// | Refuse    | `[":delete", ":kinds"]`                              |
pub(super) async fn handle_kinds_ns(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    reply_type: &str,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    if !rest.is_empty() {
        return Err(anyhow!(
            "kinds path segments not supported (protocol IDs use CBOR value args)"
        ));
    }
    match (tail, args.as_slice()) {
        // GET :kinds → list all protocol IDs
        (None, []) => {
            let manifest = load_manifest(ctx).await?;
            let ids: Vec<String> = manifest.kinds.protocol_ids();
            send_crud_reply_cbor(message, reply_type, ctx, &ids).await
        }
        // SET :kinds with [protocol_text, cid] → upsert kind;
        // SET :kinds with [protocol_text]     → delete kind
        (Some(""), [CborValue::Array(items)]) => {
            let items = items.clone();
            match items.as_slice() {
                [CborValue::Text(protocol), CborValue::Text(cid_str)] => {
                    if !is_cidv1(cid_str) {
                        return send_crud_i18n_error(message, reply_type, ctx, "cidv1-required")
                            .await;
                    }
                    let protocol = protocol.clone();
                    let cid = cid_str.clone();
                    let new_root = with_manifest_crud(ctx, |m| {
                        m.kinds.insert_protocol(&protocol, IpldLink::new(&cid));
                        Ok(())
                    })
                    .await?;
                    send_crud_ok_cid(message, reply_type, ctx, &new_root).await
                }
                [CborValue::Text(protocol)] => {
                    let protocol = protocol.clone();
                    with_manifest_crud(ctx, |m| {
                        m.kinds.remove_protocol(&protocol);
                        Ok(())
                    })
                    .await?;
                    send_crud_ok(message, reply_type, ctx).await
                }
                _ => Err(anyhow!(
                    "kinds upsert value must be [protocol_text, cid] or [protocol_text] for delete"
                )),
            }
        }
        // SET :kinds with protocol_text → get that kind's KindNode as CBOR
        (Some(""), [CborValue::Text(protocol)]) => {
            let manifest = load_manifest(ctx).await?;
            match manifest.kinds.get_protocol(protocol) {
                Some(link) => {
                    let kind: KindNode = crate::kubo::dag_get(ctx.kubo_rpc_url, &link.cid).await?;
                    send_crud_reply_cbor(message, reply_type, ctx, &kind).await
                }
                None => send_crud_i18n_error(message, reply_type, ctx, "kind-not-found").await,
            }
        }
        // DELETE :kinds (root) → refuse
        (Some(""), []) => {
            send_crud_i18n_error(message, reply_type, ctx, "refuse-delete-root").await
        }
        _ => Err(anyhow!("unknown kinds operation")),
    }
}
