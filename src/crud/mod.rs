//! `/ma/crud/0.0.1` — structured data management service.
//!
//! Single message type `application/x-ma-crud`. The operation is encoded in
//! the CBOR payload:
//!
//!   GET:    `[":get",    ":ns.key"]`
//!   SET:    `[":ns.key", value]`   — value is a scalar or `/ipfs/…` path
//!   DELETE: `[":delete", ":ns.key"]`
//!
//! All replies use `application/x-ma-crud-reply`.

mod config;
mod create;
mod entities;
mod helpers;
mod kinds;
mod namespaces;

use std::sync::Arc;

use anyhow::Result;
use ciborium::Value as CborValue;
use ma_core::{IpfsGatewayResolver, SigningKey, MESSAGE_TYPE_CRUD, MESSAGE_TYPE_CRUD_REPLY};
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
    // ACL: require "crud" capability. Owners bypass this gate unconditionally
    // so they can never be locked out of ACL management.
    let owners = ctx.stats.read().await.owners.clone();
    if !crate::acl::is_owner(&owners, &message.from) {
        check_full(acl, &message.from, &[CAP_CRUD], |_| async { Ok(vec![]) }).await?;
    }
    dispatch_management(message, ctx).await
}

async fn dispatch_management(message: &ma_core::Message, ctx: &CrudHandlerCtx<'_>) -> Result<()> {
    if message.message_type != MESSAGE_TYPE_CRUD {
        return helpers::send_crud_i18n_errorf(
            message,
            MESSAGE_TYPE_CRUD_REPLY,
            ctx,
            "wrong-crud-protocol",
            &[("type", &message.message_type)],
        )
        .await;
    }

    let path_owned: String;
    let tail_owned: Option<String>;
    let args: Vec<CborValue>;

    match helpers::decode_crud_payload(&message.payload())? {
        helpers::CrudOp::Get(path) => {
            path_owned = path;
            tail_owned = None;
            args = vec![];
        }
        helpers::CrudOp::Set(path, value) => {
            path_owned = path;
            tail_owned = Some(String::new());
            args = vec![value];
        }
        helpers::CrudOp::Delete(path) => {
            path_owned = path;
            tail_owned = Some(String::new());
            args = vec![];
        }
    }

    let reply_type = MESSAGE_TYPE_CRUD_REPLY;
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
        "acls" => namespaces::handle_root_acls(message, &rest, tail, args, reply_type, ctx).await,
        other => {
            namespaces::handle_namespace_op(message, other, &rest, tail, args, reply_type, ctx)
                .await
        }
    }
}
