//! `/ma/rpc/0.0.1` handler: entity plugin dispatch and `:ping`.

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use ma_core::{
    ipfs_add, Did, IpfsGatewayResolver, SigningKey, CONTENT_TYPE_TERM, MESSAGE_TYPE_RPC,
    MESSAGE_TYPE_RPC_REPLY,
};
use tracing::{debug, info, warn};

use crate::acl::{check_full, AclCache, AclMap, CAP_RPC};
use crate::entity::{CastInput, LocalMessage, PluginCtx, PluginKind, SendEnvelope};
use crate::plugin::EntityRegistry;
use crate::status::SharedStats;

pub const RPC_PROTOCOL_ID: &str = "/ma/rpc/0.0.1";

// ── Handler context ────────────────────────────────────────────────────────────

pub struct RpcHandlerCtx<'a> {
    pub our_did: &'a str,
    pub signing_key: &'a SigningKey,
    pub endpoint: &'a dyn ma_core::MaEndpoint,
    pub kubo_rpc_url: &'a str,
    pub resolver: Arc<IpfsGatewayResolver>,
    pub entity_registry: EntityRegistry,
    pub stats: SharedStats,
    pub acl_cache: AclCache,
}

// ── Entry point ────────────────────────────────────────────────────────────────

pub async fn handle_rpc_message(
    message: &ma_core::Message,
    acl: &AclMap,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    // Intra-runtime messages (sender = `<our_did>#<entity>`) skip the root ACL
    // transport gate — they are trusted local dispatches between entities on
    // this runtime.
    let intra_runtime = message.from.starts_with(&format!("{}#", ctx.our_did));
    if !intra_runtime {
        check_full(acl, &message.from, &[CAP_RPC], |_| async { Ok(vec![]) }).await?;
    }

    if message.message_type != MESSAGE_TYPE_RPC {
        return Err(anyhow!(
            "unsupported RPC message type '{}' on {}",
            message.message_type,
            RPC_PROTOCOL_ID,
        ));
    }

    let term: CborValue = ciborium::de::from_reader(message.payload().as_slice())
        .context("invalid CBOR in RPC message")?;

    // Fragment routing: entity plugin dispatch.
    if let Some(fragment) = extract_fragment(&message.to, ctx.our_did) {
        let ep = ctx.entity_registry.read().await.get(fragment).cloned();
        return if let Some(entity) = ep {
            match handle_entity_plugin_message(message, term, &entity, ctx).await {
                Ok(()) => Ok(()),
                Err(err) => {
                    let reason = err.to_string();
                    warn!(
                        fragment = %entity.fragment,
                        from = %message.from,
                        error = %reason,
                        "plugin dispatch rejected"
                    );
                    send_rpc_error_reply(message, ctx, &reason).await
                }
            }
        } else {
            let reason = format!("unknown entity fragment: {fragment}");
            info!(fragment = %fragment, "{}", crate::i18n::t("entity-not-found"));
            send_rpc_error_reply(message, ctx, &reason).await
        };
    }

    // Unfragmented: only :ping.
    let ping_text = match &term {
        CborValue::Text(s) => s.as_str(),
        _ => {
            return send_rpc_error_reply(message, ctx, "unfragmented RPC must be a text atom")
                .await;
        }
    };
    if ping_text == ":ping" {
        info!("{}", crate::i18n::t("ping-received"));
        let mut pong = Vec::new();
        ciborium::ser::into_writer(&CborValue::Text(":pong".to_string()), &mut pong)
            .context("encode :pong")?;
        return send_rpc_reply(message, ctx, pong).await;
    }
    send_rpc_error_reply(message, ctx, &format!("unknown RPC verb: {ping_text}")).await
}

// ── Fragment extraction ────────────────────────────────────────────────────────

/// Strip `<our_did>#` from `to` and return the bare fragment, if present.
fn extract_fragment<'a>(to: &'a str, our_did: &str) -> Option<&'a str> {
    let prefix = format!("{our_did}#");
    to.strip_prefix(prefix.as_str())
}

// ── Wasm entity dispatch ───────────────────────────────────────────────────────

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
async fn handle_entity_plugin_message(
    message: &ma_core::Message,
    term: CborValue,
    entity: &crate::plugin::EntityPlugin,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    info!(fragment = %entity.fragment, from = %message.from, "{}", crate::i18n::t("entity-dispatched"));

    // Entity verb-ACL enforcement.
    // Extract the verb from the CBOR term (text atom or first array element).
    let verb_str: Option<String> = match &term {
        CborValue::Text(s) => Some(s.clone()),
        CborValue::Array(items) => {
            if let Some(CborValue::Text(s)) = items.first() {
                Some(s.clone())
            } else {
                None
            }
        }
        _ => None,
    };

    // Empty acl field → deny-all (fail-closed). Matches EntityNode.acl doc contract.
    if entity.acl.is_empty() {
        return Err(anyhow!(
            "entity '{}' has no ACL configured: access denied (fail-closed)",
            entity.fragment
        ));
    }
    let acl_name = &entity.acl;
    let acl_key = format!("acls.{acl_name}");
    let maybe_acl = ctx.acl_cache.read().await.get(&acl_key).cloned();
    if let Some(ref acl_map) = maybe_acl {
        let verb_ref = verb_str.as_deref().unwrap_or("*");
        let root_cid = current_root_cid(ctx).await.unwrap_or_default();
        let url = ctx.kubo_rpc_url.to_string();
        let rc = root_cid.clone();
        crate::acl::check_full(acl_map, &message.from, &[verb_ref, "*"], |g| {
            let url = url.clone();
            let rc = rc.clone();
            let g = g.to_string();
            async move { crate::acl::fetch_group_members(&url, &g, &rc).await }
        })
        .await
        .with_context(|| {
            format!(
                "entity '{}' ACL denied {} calling {:?}",
                entity.fragment, message.from, verb_str
            )
        })?;
    } else {
        // ACL name set but not in cache → deny (fail-closed).
        return Err(anyhow!(
            "entity '{}' ACL '{}' not found in cache: access denied",
            entity.fragment,
            acl_name
        ));
    }

    let mut content_bytes = Vec::new();
    ciborium::ser::into_writer(&term, &mut content_bytes)
        .context("re-encoding RPC content for plugin dispatch")?;

    let local_msg = LocalMessage {
        id: message.id.clone(),
        from: message.from.clone(),
        to: message.to.clone(),
        created_at: (message.created_at * 1_000_000_000.0) as u64,
        expires: message.exp,
        reply_to: message.reply_to.clone(),
        content_type: message.content_type.clone(),
        content: content_bytes,
    };

    let plugin_ctx = PluginCtx {
        self_did: message.to.clone(),
    };

    let cast_input = CastInput {
        msg: local_msg,
        ctx: plugin_ctx,
    };
    let result = match entity.kind {
        PluginKind::Stateless => entity.handle_cast(&cast_input)?,
        PluginKind::Stateful => entity.handle_call(&cast_input)?,
    };

    for env in result.envelopes {
        if let Err(e) = send_envelope(env, ctx, &entity.fragment).await {
            warn!(fragment = %entity.fragment, error = %e, "plugin envelope delivery failed");
        }
    }

    // If the plugin called `ma_set_state` during this dispatch, persist to IPFS.
    if let Some(state_bytes) = result.pending_state {
        match ipfs_add(ctx.kubo_rpc_url, state_bytes.clone()).await {
            Ok(cid) => {
                debug!(fragment = %entity.fragment, %cid, "plugin state saved via ma_set_state");
                entity.mark_saved(state_bytes);
            }
            Err(e) => {
                warn!(fragment = %entity.fragment, error = %e, "failed to persist plugin state");
            }
        }
    }
    Ok(())
}

/// Send an outbound message produced by a plugin via the `ma_send` host function.
async fn send_envelope(env: SendEnvelope, ctx: &RpcHandlerCtx<'_>, fragment: &str) -> Result<()> {
    let recipient = Did::try_from(env.to.as_str())
        .with_context(|| format!("invalid `to` DID in plugin envelope: {}", env.to))?;

    let msg_type = if env.reply_to.is_some() {
        MESSAGE_TYPE_RPC_REPLY
    } else {
        MESSAGE_TYPE_RPC
    };

    // Include entity fragment in sender DID-URL (e.g., did:ma:ipns#fortune)
    let sender_did_url = format!("{}#{}", ctx.our_did, fragment);

    let mut msg = ma_core::Message::new(
        &sender_did_url,
        &env.to,
        msg_type,
        &env.content_type,
        &env.content,
        ctx.signing_key,
    )
    .context("failed to build plugin outbound message")?;
    msg.reply_to = env.reply_to;

    match ctx
        .endpoint
        .outbox(ctx.resolver.as_ref(), &recipient.base_id(), RPC_PROTOCOL_ID)
        .await
    {
        Ok(mut outbox) => {
            outbox
                .send(&msg)
                .await
                .context("plugin message send failed")?;
            if msg_type == MESSAGE_TYPE_RPC_REPLY {
                info!(
                    to = %env.to,
                    reply_to = ?msg.reply_to,
                    content_type = %env.content_type,
                    "{}",
                    crate::i18n::t("entity-replied")
                );
            }
        }
        Err(err) => warn!(error = %err, to = %env.to, "plugin message delivery failed"),
    }
    Ok(())
}

async fn current_root_cid(ctx: &RpcHandlerCtx<'_>) -> Result<String> {
    ctx.stats
        .read()
        .await
        .root_cid
        .clone()
        .ok_or_else(|| anyhow!("no root_cid; run --gen-root-cid first"))
}

// ── Generic reply helper ───────────────────────────────────────────────────────

async fn send_rpc_reply(
    incoming: &ma_core::Message,
    ctx: &RpcHandlerCtx<'_>,
    content: Vec<u8>,
) -> Result<()> {
    send_rpc_reply_typed(incoming, ctx, CONTENT_TYPE_TERM, &content).await
}

async fn send_rpc_reply_typed(
    incoming: &ma_core::Message,
    ctx: &RpcHandlerCtx<'_>,
    content_type: &str,
    content: &[u8],
) -> Result<()> {
    let sender = Did::try_from(incoming.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", incoming.from))?;

    let mut reply = ma_core::Message::new(
        ctx.our_did,
        &incoming.from,
        MESSAGE_TYPE_RPC_REPLY,
        content_type,
        content,
        ctx.signing_key,
    )
    .context("failed to build RPC reply")?;
    reply.reply_to = Some(incoming.id.clone());

    match ctx
        .endpoint
        .outbox(ctx.resolver.as_ref(), &sender.base_id(), RPC_PROTOCOL_ID)
        .await
    {
        Ok(mut outbox) => {
            outbox.send(&reply).await.context("RPC reply send failed")?;
            info!(
                to = %incoming.from,
                reply_to = %incoming.id,
                "{}",
                crate::i18n::t("rpc-reply-sent")
            );
        }
        Err(err) => {
            warn!(error = %err, to = %incoming.from, "RPC reply delivery failed");
        }
    }
    Ok(())
}

async fn send_rpc_error_reply(
    incoming: &ma_core::Message,
    ctx: &RpcHandlerCtx<'_>,
    reason: &str,
) -> Result<()> {
    let mut payload = Vec::new();
    ciborium::ser::into_writer(
        &CborValue::Array(vec![
            CborValue::Text(":error".to_string()),
            CborValue::Text(reason.to_string()),
        ]),
        &mut payload,
    )
    .context("failed to encode RPC error reply")?;
    send_rpc_reply(incoming, ctx, payload).await
}
