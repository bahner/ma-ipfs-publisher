//! `/ma/crud/0.0.1` — structured data management service.
//!
//! Handles four content types (get/edit/set/delete) addressed to named paths
//! in the runtime manifest tree.  See `ma-spec/ma-crud-service-v1.md`.

mod config;
mod create;
mod entities;
mod helpers;
mod kinds;
mod namespaces;

use std::sync::Arc;

use anyhow::Result;
use ciborium::Value as CborValue;
use ma_core::{
    IpfsGatewayResolver, SigningKey, MESSAGE_TYPE_CRUD_DELETE, MESSAGE_TYPE_CRUD_DELETE_REPLY,
    MESSAGE_TYPE_CRUD_EDIT, MESSAGE_TYPE_CRUD_EDIT_REPLY, MESSAGE_TYPE_CRUD_GET,
    MESSAGE_TYPE_CRUD_GET_REPLY, MESSAGE_TYPE_CRUD_SET, MESSAGE_TYPE_CRUD_SET_REPLY,
};
use tokio::sync::RwLock;

use crate::acl::{check_full, AclCache, AclMap, SharedAcl, CAP_CRUD};
use crate::plugin::EntityRegistry;
use crate::status::SharedStats;

pub use ma_core::CRUD_PROTOCOL_ID;

// ── Handler context ────────────────────────────────────────────────────────────

pub struct CrudHandlerCtx<'a> {
    pub our_did: &'a str,
    pub signing_key: &'a SigningKey,
    pub endpoint: &'a dyn ma_core::MaEndpoint,
    pub kubo_rpc_url: &'a str,
    pub resolver: Arc<IpfsGatewayResolver>,
    pub stats: SharedStats,
    pub entity_registry: EntityRegistry,
    pub shared_config: Arc<RwLock<ma_core::Config>>,
    /// Namespace ACL cache — maps `"<ns>.acl"` and `"<ns>.acls.<name>"` to their
    /// `AclMap`s for zero-overhead lookup at call time.
    pub acl_cache: AclCache,
    /// Shared root transport ACL — owner may update at runtime via `:acl: <cid>`.
    pub root_acl: SharedAcl,
}

// ── Entry point ────────────────────────────────────────────────────────────────

pub async fn handle_crud_message(
    message: &ma_core::Message,
    acl: &AclMap,
    ctx: &CrudHandlerCtx<'_>,
) -> Result<()> {
    // ACL: require "crud" capability.
    check_full(acl, &message.from, &[CAP_CRUD], |_| async { Ok(vec![]) }).await?;
    dispatch_management(message, ctx).await
}

async fn dispatch_management(message: &ma_core::Message, ctx: &CrudHandlerCtx<'_>) -> Result<()> {
    let path_owned: String;
    let tail_owned: Option<String>;
    let args: Vec<CborValue>;
    let reply_type: &str;

    match message.message_type.as_str() {
        MESSAGE_TYPE_CRUD_GET => {
            path_owned = helpers::decode_path_atom(&message.payload())?;
            tail_owned = None;
            args = vec![];
            reply_type = MESSAGE_TYPE_CRUD_GET_REPLY;
        }
        MESSAGE_TYPE_CRUD_EDIT => {
            path_owned = helpers::decode_path_atom(&message.payload())?;
            tail_owned = Some("edit".to_string());
            args = vec![];
            reply_type = MESSAGE_TYPE_CRUD_EDIT_REPLY;
        }
        MESSAGE_TYPE_CRUD_SET => {
            let (p, v) = helpers::decode_set_payload(&message.payload())?;
            path_owned = p;
            tail_owned = Some(String::new());
            args = vec![v];
            reply_type = MESSAGE_TYPE_CRUD_SET_REPLY;
        }
        MESSAGE_TYPE_CRUD_DELETE => {
            path_owned = helpers::decode_path_atom(&message.payload())?;
            tail_owned = Some(String::new());
            args = vec![];
            reply_type = MESSAGE_TYPE_CRUD_DELETE_REPLY;
        }
        other => {
            return helpers::send_crud_error(
                message,
                MESSAGE_TYPE_CRUD_GET_REPLY,
                ctx,
                &format!("wrong-protocol: {other}"),
            )
            .await;
        }
    }

    let tail: Option<&str> = tail_owned.as_deref();
    let (ns, rest) = helpers::parse_path(&path_owned)?;

    match ns {
        "entities" => {
            entities::handle_entities_ns(message, &rest, tail, args, reply_type, ctx).await
        }
        "kinds" => kinds::handle_kinds_ns(message, &rest, tail, args, reply_type, ctx).await,
        "config" => config::handle_config_ns(message, &rest, tail, args, reply_type, ctx).await,
        "create" => create::handle_create_ns(message, tail, args, reply_type, ctx).await,
        "acl" => namespaces::handle_root_acl(message, tail, args, reply_type, ctx).await,
        other => {
            namespaces::handle_namespace_op(message, other, &rest, tail, args, reply_type, ctx)
                .await
        }
    }
}
