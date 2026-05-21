//! `/ma/rpc/0.0.1` handler: entity dispatch and CRUD management.

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use ma_core::{
    ipfs_add, Did, IpfsGatewayResolver, SigningKey, MESSAGE_TYPE_RPC, MESSAGE_TYPE_RPC_REPLY,
};
use tracing::{debug, info, warn};

use crate::acl::{check_full, AclCache, AclMap, SharedAcl, CAP_RPC};
use crate::entity::{
    CastInput, EntityNode, IpldLink, LocalMessage, NamespaceNode, PluginCtx, PluginKind,
    RuntimeManifest, SendEnvelope,
};
use crate::plugin::EntityRegistry;
use crate::status::SharedStats;

pub const RPC_PROTOCOL_ID: &str = "/ma/rpc/0.0.1";

// ── Handler context ────────────────────────────────────────────────────────────

pub struct RpcHandlerCtx<'a> {
    pub our_did: &'a str,
    pub signing_key: &'a SigningKey,
    pub endpoint: &'a dyn ma_core::MaEndpoint,
    pub kubo_rpc_url: &'a str,
    pub entity_registry: EntityRegistry,
    pub stats: SharedStats,
    pub acl_cache: AclCache,
    /// Shared root transport ACL — owner may update at runtime via `:acl`.
    pub root_acl: SharedAcl,
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
    let intra_runtime = message
        .from
        .starts_with(&format!("{}#", ctx.our_did));
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

    // Fragment routing: messages addressed to `did:ma:<ipns>#fragment`.
    //
    // Two cases:
    //   - Management term (":", or [":", cid]) → entity CRUD (register/delete).
    //   - All other terms → entity verb dispatch.
    if let Some(fragment) = extract_fragment(&message.to, ctx.our_did) {
        if is_entity_management_term(&term) {
            // Route fragment-addressed CRUD to the entities namespace handler.
            // The fragment is the entity name; verb/args are extracted below.
            return handle_fragment_entity_crud(message, fragment, term, ctx).await;
        }
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

    // Unfragmented: CRUD management verbs (entities/kinds/config).
    handle_root_builtin(message, term, ctx).await
}

// ── Fragment extraction ────────────────────────────────────────────────────────

/// Strip `<our_did>#` from `to` and return the bare fragment, if present.
fn extract_fragment<'a>(to: &'a str, our_did: &str) -> Option<&'a str> {
    let prefix = format!("{our_did}#");
    to.strip_prefix(prefix.as_str())
}

/// Returns `true` when the CBOR term represents a CRUD management operation
/// rather than an entity verb call.
///
/// Convention: a term whose verb atom (first element of a tuple, or the atom
/// itself) is exactly `":"` (empty path after stripping the leading `:`) is a
/// CRUD operation:
/// - `":"` alone → delete entity
/// - `[":", <cid>]` → upsert entity to `<cid>`
///
/// Any other atom (e.g. `":ping"`, `":cast"`) is an entity verb call.
fn is_entity_management_term(term: &CborValue) -> bool {
    match term {
        CborValue::Text(s) => s == ":",
        CborValue::Array(items) => {
            matches!(items.first(), Some(CborValue::Text(s)) if s == ":")
        }
        _ => false,
    }
}

/// Handle fragment-addressed CRUD: `@runtime#fortune:` (delete) or
/// `@runtime#fortune: <cid>` (upsert). Delegates to the same logic as
/// `:entities.<name>:` / `[:entities.<name>:, <cid>]` unfragmented CRUD.
async fn handle_fragment_entity_crud(
    message: &ma_core::Message,
    fragment: &str,
    term: CborValue,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    let (verb, args) = parse_cbor_verb(term)?;
    let name = fragment.to_string();
    // verb is ":" for CRUD; tail is empty string (delete/set)
    handle_single_entity(message, &name, Some(""), args, &verb, ctx).await
}

// ── CBOR verb parsing ──────────────────────────────────────────────────────────

/// Decode `CborValue::Text(":verb")` or `CborValue::Array([":verb", args…])`
/// into `(verb, args)`.
fn parse_cbor_verb(term: CborValue) -> Result<(String, Vec<CborValue>)> {
    Ok(match term {
        CborValue::Text(s) => (s, vec![]),
        CborValue::Array(items) => {
            let mut it = items.into_iter();
            let Some(CborValue::Text(verb)) = it.next() else {
                return Err(anyhow!("RPC array must start with a text verb atom"));
            };
            (verb, it.collect())
        }
        _ => return Err(anyhow!("RPC payload must be a CBOR text atom or array")),
    })
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

    let resolver = IpfsGatewayResolver::new(ctx.kubo_rpc_url.to_string());
    match ctx
        .endpoint
        .outbox(&resolver, &recipient.base_id(), RPC_PROTOCOL_ID)
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

// ── Dot-path operation parser ──────────────────────────────────────────────────

/// Parse `":entities.ping:"` into path segments and optional tail.
///
/// - `":entities.ping:"` → `(["entities","ping"], Some(""))` — set/delete
/// - `":entities.ping:edit"` → `(["entities","ping"], Some("edit"))` — verb
/// - `":entities.ping"` → `(["entities","ping"], None)` — get
fn parse_dot_op(verb: &str) -> (Vec<String>, Option<String>) {
    let body = verb.strip_prefix(':').unwrap_or(verb);
    let (path_part, tail) = body.find(':').map_or((body, None), |pos| {
        (&body[..pos], Some(body[pos + 1..].to_string()))
    });
    let segs = path_part
        .split('.')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    (segs, tail)
}

/// Fetch the manifest, apply `f` to mutate it, re-store, and pin-swap.
/// Returns the new root CID.
async fn with_manifest<F>(ctx: &RpcHandlerCtx<'_>, f: F) -> Result<String>
where
    F: FnOnce(&mut RuntimeManifest) -> Result<()>,
{
    let old_cid = current_root_cid(ctx).await?;
    let mut manifest: RuntimeManifest = crate::kubo::dag_get(ctx.kubo_rpc_url, &old_cid).await?;
    f(&mut manifest)?;
    let new_cid = crate::kubo::dag_put(ctx.kubo_rpc_url, &manifest).await?;
    if let Err(e) = crate::kubo::pin_update(ctx.kubo_rpc_url, &old_cid, &new_cid).await {
        warn!(old = %old_cid, new = %new_cid, error = %e, "pin/update failed");
    }
    update_stats_entities(ctx).await;
    ctx.stats.write().await.root_cid = Some(new_cid.clone());
    Ok(new_cid)
}

// ── Built-in #root handlers ────────────────────────────────────────────────────

async fn handle_root_builtin(
    message: &ma_core::Message,
    term: CborValue,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    let (verb, args) = parse_cbor_verb(term)?;
    let (segs, tail_owned) = parse_dot_op(&verb);
    let tail: Option<&str> = tail_owned.as_deref();
    let ns: &str = segs.first().map_or("", String::as_str);
    let rest: &[String] = if segs.len() > 1 { &segs[1..] } else { &[] };

    match ns {
        "entities" => handle_entities_ns(message, rest, tail, args, &verb, ctx).await,
        "kinds" => handle_kinds_ns(message, rest, tail, args, &verb, ctx).await,
        "config" => handle_config_ns(message, rest, tail, args, &verb, ctx).await,
        "ping" => {
            info!("{}", crate::i18n::t("ping-received"));
            let mut pong = Vec::new();
            ciborium::ser::into_writer(&CborValue::Text(":pong".to_string()), &mut pong)
                .context("encode :pong")?;
            send_rpc_reply(message, ctx, pong).await
        }
        other => handle_namespace_op(message, other, rest, tail, args, &verb, ctx).await,
    }
}

async fn handle_entities_ns(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    match rest.len() {
        0 => match (tail, args.as_slice()) {
            (None, []) => {
                info!("{}", crate::i18n::t("root-list-entities"));
                let names: Vec<String> = ctx.entity_registry.read().await.keys().cloned().collect();
                let mut out = Vec::new();
                ciborium::ser::into_writer(&names, &mut out)
                    .context("encoding entity list as CBOR")?;
                send_rpc_reply(message, ctx, out).await
            }
            (Some(""), _) => {
                // The :entities root is protected — silently ignore delete attempts.
                send_rpc_ok_reply(message, ctx).await
            }
            _ => Err(anyhow!("unknown entities operation: {verb}")),
        },
        1 => handle_single_entity(message, &rest[0], tail, args, verb, ctx).await,
        2.. => handle_entity_field(message, &rest[0], &rest[1..], tail, args, verb, ctx).await,
    }
}

#[allow(clippy::too_many_lines)]
async fn handle_single_entity(
    message: &ma_core::Message,
    name: &String,
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let root_cid = current_root_cid(ctx).await?;
            let manifest: RuntimeManifest =
                crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
            let link = manifest
                .entities
                .get(name.as_str())
                .ok_or_else(|| anyhow!("entity not found: {name}"))?;
            let entity: EntityNode = crate::kubo::dag_get(ctx.kubo_rpc_url, &link.cid).await?;
            let mut out = Vec::new();
            ciborium::ser::into_writer(&entity, &mut out)
                .context("serialising entity node as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        (Some(""), []) => {
            // Delete entity — requires `delete` + `entities` in root ACL.
            check_entity_management_cap(message, ctx, &["delete", "entities"]).await?;
            let name = name.as_str();
            ctx.entity_registry.write().await.remove(name);
            let new_root = with_manifest(ctx, |m| {
                m.entities.remove(name);
                Ok(())
            })
            .await?;
            info!(name = %name, cid = %new_root, "{}", crate::i18n::t("entity-deleted"));
            let mut out = Vec::new();
            ciborium::ser::into_writer(&CborValue::Text(":ok".to_string()), &mut out)
                .context("encoding delete reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        (Some(""), [CborValue::Text(path)]) => {
            // Upsert entity — requires `create` + `entities` in root ACL.
            check_entity_management_cap(message, ctx, &["create", "entities"]).await?;
            let name = name.as_str();
            // Accept bare CIDs, /ipfs/<cid>, and /ipns/<key> paths.
            let cid = crate::kubo::dag_resolve(ctx.kubo_rpc_url, path)
                .await
                .with_context(|| format!("resolving path {path}"))?;
            let cid = cid.as_str();
            let entity_node: EntityNode = crate::kubo::dag_get(ctx.kubo_rpc_url, cid)
                .await
                .with_context(|| format!("fetching entity node from {cid}"))?;
            let new_root = with_manifest(ctx, |m| {
                m.entities.insert(name.to_string(), IpldLink::new(cid));
                Ok(())
            })
            .await?;
            match crate::plugin::EntityPlugin::load(
                name.to_string(),
                &entity_node,
                ctx.kubo_rpc_url,
            )
            .await
            {
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
            info!(name = %name, cid = %cid, "{}", crate::i18n::t("entity-created"));
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(new_root),
                ]),
                &mut out,
            )
            .context("encoding upsert reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        (Some("edit"), []) => {
            let root_cid = current_root_cid(ctx).await?;
            let manifest: RuntimeManifest =
                crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
            let entity: EntityNode = match manifest.entities.get(name.as_str()) {
                Some(link) => crate::kubo::dag_get(ctx.kubo_rpc_url, &link.cid).await?,
                None => EntityNode {
                    kind: String::new(),
                    behavior: String::new(),
                    acl: String::new(),
                    state: None,
                },
            };
            let mut out = Vec::new();
            ciborium::ser::into_writer(&entity, &mut out).context("encoding entity as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        (Some("edit"), [CborValue::Bytes(dag_cbor)]) => {
            let name = name.as_str();
            let cid = crate::kubo::dag_put_raw(ctx.kubo_rpc_url, dag_cbor)
                .await
                .with_context(|| format!("dag_put_raw for entity {name}"))?;
            let entity_node: EntityNode = crate::kubo::dag_get(ctx.kubo_rpc_url, &cid)
                .await
                .with_context(|| format!("validating entity node at {cid}"))?;
            with_manifest(ctx, |m| {
                m.entities.insert(name.to_string(), IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            match crate::plugin::EntityPlugin::load(
                name.to_string(),
                &entity_node,
                ctx.kubo_rpc_url,
            )
            .await
            {
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
            info!(name = %name, cid = %cid, "{}", crate::i18n::t("entity-created"));
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(cid),
                ]),
                &mut out,
            )
            .context("encoding edit-save reply CID")?;
            send_rpc_reply(message, ctx, out).await
        }
        _ => Err(anyhow!("unknown entities.{name} operation: {verb}")),
    }
}

// ── Entity field helpers ──────────────────────────────────────────────────────

/// Fetch an `EntityNode` by name from the current manifest.
async fn fetch_entity_node(ctx: &RpcHandlerCtx<'_>, name: &str) -> Result<EntityNode> {
    let root_cid = current_root_cid(ctx).await?;
    let manifest: RuntimeManifest = crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
    let link = manifest
        .entities
        .get(name)
        .ok_or_else(|| anyhow!("entity not found: {name}"))?;
    crate::kubo::dag_get(ctx.kubo_rpc_url, &link.cid)
        .await
        .with_context(|| format!("fetching entity {name} from {}", link.cid))
}

/// Re-publish a mutated `EntityNode` and point the manifest at the new CID.
/// Returns the new entity CID.
async fn update_entity_node(
    ctx: &RpcHandlerCtx<'_>,
    name: &str,
    entity: &EntityNode,
) -> Result<String> {
    let entity_cid = crate::kubo::dag_put(ctx.kubo_rpc_url, entity)
        .await
        .with_context(|| format!("publishing updated entity {name}"))?;
    with_manifest(ctx, |m| {
        m.entities
            .insert(name.to_string(), IpldLink::new(&entity_cid));
        Ok(())
    })
    .await?;
    Ok(entity_cid)
}

/// Dispatch `entities.<name>.<field>[.<sub>…]` sub-path operations.
///
/// Peels off the first segment and routes to the field handler, which receives
/// the remaining sub-path.  Adding support for a new field or deeper sub-path
/// only requires changes in the relevant handler — this function stays stable.
async fn handle_entity_field(
    message: &ma_core::Message,
    name: &String,
    field_path: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    let Some((field, sub_path)) = field_path.split_first() else {
        return Err(anyhow!("empty field path in {verb}"));
    };

    // Generic GET / :edit — works for any leaf field without field-specific code.
    if matches!(tail, None | Some("edit")) && args.is_empty() && sub_path.is_empty() {
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
                return send_rpc_reply(message, ctx, out).await;
            }
        }
        return Err(anyhow!("field '{field}' not found in entity '{name}'"));
    }

    match field.as_str() {
        "acl" => handle_entity_acl_field(message, name, sub_path, tail, args, verb, ctx).await,
        _ => Err(anyhow!("unknown entity field '{field}' in {verb}")),
    }
}

async fn handle_entity_acl_field(
    message: &ma_core::Message,
    name: &String,
    sub_path: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    if !sub_path.is_empty() {
        return Err(anyhow!(
            "entity field 'acl' sub-path '{}' not yet implemented",
            sub_path.join(".")
        ));
    }
    match (tail, args.as_slice()) {
        // No args — return current ACL name string.
        (None, []) => {
            let entity = fetch_entity_node(ctx, name).await?;
            let mut out = Vec::new();
            ciborium::ser::into_writer(&CborValue::Text(entity.acl.clone()), &mut out)
                .context("encoding entity ACL name as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        // : <name> — set ACL name string (references root `acls.<name>`).
        (Some(""), [CborValue::Text(acl_name)]) => {
            let mut entity = fetch_entity_node(ctx, name).await?;
            entity.acl = acl_name.clone();
            let entity_cid = update_entity_node(ctx, name, &entity).await?;
            info!(name = %name, acl_name = %acl_name, entity_cid = %entity_cid, "entity ACL name set");
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(entity_cid),
                ]),
                &mut out,
            )
            .context("encoding acl-set reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        // : — clear ACL name (entity becomes deny-all).
        (Some(""), []) => {
            let mut entity = fetch_entity_node(ctx, name).await?;
            entity.acl = String::new();
            let entity_cid = update_entity_node(ctx, name, &entity).await?;
            info!(name = %name, entity_cid = %entity_cid, "entity ACL cleared");
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(entity_cid),
                ]),
                &mut out,
            )
            .context("encoding acl-clear reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        _ => Err(anyhow!("unknown entities.{name}.acl operation: {verb}")),
    }
}

/// Handle `:kinds` RPC operations.
///
/// Protocol IDs contain slashes and cannot appear as dot-path segments.
/// All kinds operations use tuple args:
///
/// | Term                                              | Description  |
/// |---------------------------------------------------|--------------|
/// | `:kinds`                                          | list all IDs |
/// | `[":kinds", "<protocol>"]`                        | get KindNode |
/// | `[":kinds:", "<protocol>"]`                       | delete       |
/// | `[":kinds:", "<protocol>", <dag-cbor-bytes>]`     | upsert       |
async fn handle_kinds_ns(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    if !rest.is_empty() {
        return Err(anyhow!(
            "unknown kinds operation: {verb} \
             (protocol IDs are tuple args, not dot-path segments)"
        ));
    }
    match (tail, args.as_slice()) {
        // :kinds → list all protocol IDs
        (None, []) => {
            let root_cid = current_root_cid(ctx).await?;
            let manifest: RuntimeManifest =
                crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
            let ids: Vec<&str> = manifest.kinds.keys().map(String::as_str).collect();
            let mut out = Vec::new();
            ciborium::ser::into_writer(&ids, &mut out)
                .context("encoding kinds list as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        // [":kinds", "<protocol>"] → get KindNode as CBOR
        (None, [CborValue::Text(protocol)]) => {
            let root_cid = current_root_cid(ctx).await?;
            let manifest: RuntimeManifest =
                crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
            let val = manifest
                .kinds
                .get(protocol.as_str())
                .ok_or_else(|| anyhow!("kind not found: {protocol}"))?;
            let mut out = Vec::new();
            ciborium::ser::into_writer(val, &mut out).context("encoding kind as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        // [":kinds:", "<protocol>"] → delete
        (Some(""), [CborValue::Text(protocol)]) => {
            let protocol = protocol.clone();
            with_manifest(ctx, |m| {
                m.kinds.remove(&protocol);
                Ok(())
            })
            .await?;
            send_rpc_ok_reply(message, ctx).await
        }
        // [":kinds:", "<protocol>", <dag-cbor-bytes>] → upsert
        (Some(""), [CborValue::Text(protocol), CborValue::Bytes(cbor_bytes)]) => {
            use crate::entity::{IpldLink as Link, KindRef};
            let protocol = protocol.clone();
            let cid = crate::kubo::dag_put_raw(ctx.kubo_rpc_url, cbor_bytes)
                .await
                .context("dag_put kind")?;
            let new_root = with_manifest(ctx, |m| {
                m.kinds.insert(protocol, KindRef::Link(Link::new(&cid)));
                Ok(())
            })
            .await?;
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(new_root),
                ]),
                &mut out,
            )
            .context("encoding kinds-upsert reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        // [":kinds:"] → delete root (protected — silently accept)
        (Some(""), []) => send_rpc_ok_reply(message, ctx).await,
        _ => Err(anyhow!("unknown kinds operation: {verb}")),
    }
}

async fn handle_config_ns(
    message: &ma_core::Message,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    match rest {
        [] => match (tail, args.as_slice()) {
            (None | Some("edit"), []) => {
                let root_cid = current_root_cid(ctx).await?;
                let manifest: RuntimeManifest =
                    crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
                let mut out = Vec::new();
                ciborium::ser::into_writer(&manifest.config, &mut out)
                    .context("encoding config as CBOR")?;
                send_rpc_reply(message, ctx, out).await
            }
            (Some(""), _) => {
                // The :config root is protected — silently ignore delete attempts.
                send_rpc_ok_reply(message, ctx).await
            }
            _ => Err(anyhow!("unknown config operation: {verb}")),
        },
        [key] => match (tail, args.as_slice()) {
            (None | Some("edit"), []) => {
                let root_cid = current_root_cid(ctx).await?;
                let manifest: RuntimeManifest =
                    crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
                let val = manifest
                    .config
                    .get(key.as_str())
                    .ok_or_else(|| anyhow!("config key not found: {key}"))?;
                let mut out = Vec::new();
                ciborium::ser::into_writer(val, &mut out)
                    .context("encoding config value as CBOR")?;
                send_rpc_reply(message, ctx, out).await
            }
            (Some(""), []) => {
                let key = key.as_str().to_string();
                with_manifest(ctx, |m| {
                    m.config.remove(&key);
                    Ok(())
                })
                .await?;
                let mut out = Vec::new();
                ciborium::ser::into_writer(&CborValue::Text(":ok".to_string()), &mut out)
                    .context("encoding config-delete reply as CBOR")?;
                send_rpc_reply(message, ctx, out).await
            }
            (Some(""), [CborValue::Text(value)]) => {
                let key = key.as_str().to_string();
                let json_val: serde_json::Value = serde_json::from_str(value.as_str())
                    .unwrap_or_else(|_| serde_json::Value::String(value.clone()));
                let new_root = with_manifest(ctx, |m| {
                    m.config.insert(key, json_val);
                    Ok(())
                })
                .await?;
                let mut out = Vec::new();
                ciborium::ser::into_writer(
                    &CborValue::Array(vec![
                        CborValue::Text(":ok".to_string()),
                        CborValue::Text(new_root),
                    ]),
                    &mut out,
                )
                .context("encoding config-set reply as CBOR")?;
                send_rpc_reply(message, ctx, out).await
            }
            _ => Err(anyhow!("unknown config.{key} operation: {verb}")),
        },
        _ => Err(anyhow!("unknown config operation: {verb}")),
    }
}

// ── Namespace dispatching `:ns.*` ─────────────────────────────────────────────

/// Handles that may not be used as namespace names.
const RESERVED_NS: &[&str] = &["acl", "acls", "protocol", "kinds", "entities", "lang", "config"];

/// Check the namespace gate ACL for `caller` against `caps`.
///
/// Looks up `"<ns>.acl"` in the cache. If no gate is registered for the
/// namespace, access is denied (fail-closed). On allow the function returns
/// `Ok(())`; on deny it returns `Err`.
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

/// Routes `:ns`, `:ns.acl`, `:ns.acls.*`, and blob operations.
async fn handle_namespace_op(
    message: &ma_core::Message,
    ns: &str,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    if ns.is_empty() {
        debug!(verb = %verb, "{}", crate::i18n::t("unknown-rpc-atom"));
        return Ok(());
    }
    let category = rest.first().map_or("", String::as_str);
    let sub_rest: &[String] = if rest.len() > 1 { &rest[1..] } else { &[] };

    // Gate check applies to blob operations only (not acl/acls management or ns root).
    if matches!(category, "" | "acl" | "acls") {
        // ACL management and namespace root are not gated.
    } else {
        // Blob op — enforce namespace gate.
        if let Ok(root_cid) = current_root_cid(ctx).await {
            let is_read = tail.is_none();
            let caps: &[&str] = if is_read { &[category, "read", "*"] } else { &[category, "update", "*"] };
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
                return send_rpc_error_reply(message, ctx, &e.to_string()).await;
            }
        }
    }

    match category {
        "acl" => handle_ns_acl_gate(message, ns, tail, args, verb, ctx).await,
        "acls" => handle_ns_acls(message, ns, sub_rest, tail, args, verb, ctx).await,
        "" => handle_ns_root(message, ns, tail, args, verb, ctx).await,
        key => handle_ns_blob(message, ns, key, sub_rest, tail, args, verb, ctx).await,
    }
}

/// `:ns` \u2014 namespace root: describe or create.
async fn handle_ns_root(
    message: &ma_core::Message,
    ns: &str,
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let root_cid = current_root_cid(ctx).await?;
            let manifest: RuntimeManifest =
                crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
            match manifest.namespaces.get(ns) {
                Some(ns_node) => {
                    let mut out = Vec::new();
                    ciborium::ser::into_writer(ns_node, &mut out)
                        .context("encoding namespace node as CBOR")?;
                    send_rpc_reply(message, ctx, out).await
                }
                None => {
                    send_rpc_error_reply(message, ctx, &format!("namespace not found: {ns}")).await
                }
            }
        }
        // Create / upsert namespace: `[:ns:]`
        (Some(""), []) => {
            if RESERVED_NS.contains(&ns) {
                // Reserved roots are protected — silently ignore.
                return send_rpc_ok_reply(message, ctx).await;
            }
            let new_root = with_manifest(ctx, |m| {
                m.namespaces
                    .entry(ns.to_string())
                    .or_insert_with(NamespaceNode::default);
                Ok(())
            })
            .await?;
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(new_root),
                ]),
                &mut out,
            )
            .context("encoding namespace-create reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        _ => Err(anyhow!("unknown namespace '{ns}' operation: {verb}")),
    }
}

/// `:ns.acl` — namespace gate: single IPLD link to an `AclMap` document.
///
/// GET returns the gate CID. SET registers a new gate and reloads the cache.
/// DELETE removes the gate (namespace becomes inaccessible to blob ops).
async fn handle_ns_acl_gate(
    message: &ma_core::Message,
    ns: &str,
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let root_cid = current_root_cid(ctx).await?;
            let manifest: RuntimeManifest =
                crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
            let ns_node = manifest
                .namespaces
                .get(ns)
                .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
            match &ns_node.acl {
                Some(link) => {
                    let mut out = Vec::new();
                    ciborium::ser::into_writer(&CborValue::Text(link.cid.clone()), &mut out)
                        .context("encoding ACL gate CID as CBOR")?;
                    send_rpc_reply(message, ctx, out).await
                }
                None => {
                    send_rpc_error_reply(message, ctx, &format!("no gate ACL for namespace {ns}"))
                        .await
                }
            }
        }
        (Some(""), [CborValue::Text(cid)]) => {
            let cid = cid.clone();
            let new_root = with_manifest(ctx, |m| {
                let ns_node = m
                    .namespaces
                    .get_mut(ns)
                    .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                ns_node.acl = Some(IpldLink::new(&cid));
                Ok(())
            })
            .await?;
            let cache_key = format!("{ns}.acl");
            match crate::acl::load_acl_from_cid(ctx.kubo_rpc_url, &cid).await {
                Ok(acl_map) => {
                    ctx.acl_cache.write().await.insert(cache_key.clone(), acl_map);
                    info!(key = %cache_key, cid = %cid, "Namespace gate ACL loaded into cache");
                }
                Err(e) => {
                    warn!(key = %cache_key, cid = %cid, error = %e, "failed to load namespace gate ACL into cache");
                }
            }
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(new_root),
                ]),
                &mut out,
            )
            .context("encoding acl-gate-set reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        (Some(""), []) => {
            let new_root = with_manifest(ctx, |m| {
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
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(new_root),
                ]),
                &mut out,
            )
            .context("encoding acl-gate-delete reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        _ => Err(anyhow!("unknown {ns}.acl operation: {verb}")),
    }
}

/// `:ns.acls.*` — named verb-ACL library for the namespace.
///
/// ACLs are stored as IPLD links to `kind: /ma/acl/0.0.1` documents and
/// cached in memory as [`AclMap`]s for zero-overhead lookup at call time.
/// Edit and publish to IPFS, then register the CID here.
#[allow(clippy::too_many_lines)]
async fn handle_ns_acls(
    message: &ma_core::Message,
    ns: &str,
    rest: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    match rest {
        [] => match (tail, args.as_slice()) {
            (None, []) => {
                let root_cid = current_root_cid(ctx).await?;
                let manifest: RuntimeManifest =
                    crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
                let ns_node = manifest
                    .namespaces
                    .get(ns)
                    .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                let names: Vec<CborValue> = ns_node
                    .acls
                    .keys()
                    .map(|k| CborValue::Text(k.clone()))
                    .collect();
                let mut out = Vec::new();
                ciborium::ser::into_writer(
                    &CborValue::Array(vec![
                        CborValue::Text(":ok".to_string()),
                        CborValue::Array(names),
                    ]),
                    &mut out,
                )
                .context("encoding ACL names as CBOR")?;
                send_rpc_reply(message, ctx, out).await
            }
            _ => Err(anyhow!("unknown {ns}.acls operation: {verb}")),
        },
        [acl_name] => {
            let acl_name = acl_name.clone();
            match (tail, args.as_slice()) {
                (None, []) => {
                    let root_cid = current_root_cid(ctx).await?;
                    let manifest: RuntimeManifest =
                        crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
                    let ns_node = manifest
                        .namespaces
                        .get(ns)
                        .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                    let link = ns_node
                        .acls
                        .get(&acl_name)
                        .ok_or_else(|| anyhow!("ACL not found: {ns}.acls.{acl_name}"))?;
                    let mut out = Vec::new();
                    ciborium::ser::into_writer(&CborValue::Text(link.cid.clone()), &mut out)
                        .context("encoding ACL CID as CBOR")?;
                    send_rpc_reply(message, ctx, out).await
                }
                (Some(""), [CborValue::Text(cid)]) => {
                    let cid = cid.clone();
                    let new_root = with_manifest(ctx, |m| {
                        let ns_node = m
                            .namespaces
                            .get_mut(ns)
                            .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                        ns_node.acls.insert(acl_name.clone(), IpldLink::new(&cid));
                        Ok(())
                    })
                    .await?;
                    let cache_key = format!("{ns}.acls.{acl_name}");
                    match crate::acl::load_acl_from_cid(ctx.kubo_rpc_url, &cid).await {
                        Ok(acl_map) => {
                            ctx.acl_cache
                                .write()
                                .await
                                .insert(cache_key.clone(), acl_map);
                            info!(key = %cache_key, cid = %cid, "ACL loaded into cache");
                        }
                        Err(e) => {
                            warn!(key = %cache_key, cid = %cid, error = %e, "failed to load ACL into cache; CID registered but cache not updated");
                        }
                    }
                    let mut out = Vec::new();
                    ciborium::ser::into_writer(
                        &CborValue::Array(vec![
                            CborValue::Text(":ok".to_string()),
                            CborValue::Text(new_root),
                        ]),
                        &mut out,
                    )
                    .context("encoding acl-set reply as CBOR")?;
                    send_rpc_reply(message, ctx, out).await
                }
                (Some(""), []) => {
                    let new_root = with_manifest(ctx, |m| {
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
                    let mut out = Vec::new();
                    ciborium::ser::into_writer(
                        &CborValue::Array(vec![
                            CborValue::Text(":ok".to_string()),
                            CborValue::Text(new_root),
                        ]),
                        &mut out,
                    )
                    .context("encoding acl-delete reply as CBOR")?;
                    send_rpc_reply(message, ctx, out).await
                }
                _ => Err(anyhow!("unknown {ns}.acls.{acl_name} operation: {verb}")),
            }
        }
        _ => Err(anyhow!("unknown {ns}.acls operation: {verb}")),
    }
}

/// `:ns.<key>[.<sub>…]` — IPLD link store with lazy nested traversal.
///
/// Any namespace key other than `acl` is a plain IPLD link stored as
/// `{"/": "bafy…"}` in the namespace `extra` map.  The runtime does not
/// inspect or validate the linked content.
///
/// - **GET** at any depth: look up the root CID, then follow
///   `sub_path` segments via `ipfs dag resolve`.
/// - **SET / DELETE** only at depth 1 (directly in `extra`).
///   Nested structures are managed externally by the namespace owner.
#[allow(clippy::too_many_arguments)]
async fn handle_ns_blob(
    message: &ma_core::Message,
    ns: &str,
    key: &str,
    sub_path: &[String],
    tail: Option<&str>,
    args: Vec<CborValue>,
    verb: &str,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    match (tail, args.as_slice()) {
        (None, []) => {
            let root_cid = current_root_cid(ctx).await?;
            let manifest: RuntimeManifest =
                crate::kubo::dag_get(ctx.kubo_rpc_url, &root_cid).await?;
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
            let mut out = Vec::new();
            ciborium::ser::into_writer(&CborValue::Text(resolved_cid), &mut out)
                .context("encoding blob CID as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        (Some(""), [CborValue::Text(cid)]) if sub_path.is_empty() => {
            let cid = cid.clone();
            let new_root = with_manifest(ctx, |m| {
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
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(new_root),
                ]),
                &mut out,
            )
            .context("encoding blob-set reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        (Some(""), []) if sub_path.is_empty() => {
            let new_root = with_manifest(ctx, |m| {
                let ns_node = m
                    .namespaces
                    .get_mut(ns)
                    .ok_or_else(|| anyhow!("namespace not found: {ns}"))?;
                ns_node.extra.remove(key);
                Ok(())
            })
            .await?;
            info!(ns = %ns, key = %key, "blob deleted");
            let mut out = Vec::new();
            ciborium::ser::into_writer(
                &CborValue::Array(vec![
                    CborValue::Text(":ok".to_string()),
                    CborValue::Text(new_root),
                ]),
                &mut out,
            )
            .context("encoding blob-delete reply as CBOR")?;
            send_rpc_reply(message, ctx, out).await
        }
        _ => Err(anyhow!("unknown {ns}.{key} operation: {verb}")),
    }
}

async fn current_root_cid(ctx: &RpcHandlerCtx<'_>) -> Result<String> {
    ctx.stats
        .read()
        .await
        .root_cid
        .clone()
        .ok_or_else(|| anyhow!("no root_cid; run --gen-root-cid first"))
}

/// Check that `message.from` holds all `caps` in the root ACL.
/// Used for entity and kinds management operations.
async fn check_entity_management_cap(
    message: &ma_core::Message,
    ctx: &RpcHandlerCtx<'_>,
    caps: &[&str],
) -> Result<()> {
    let acl = ctx.root_acl.read().await;
    check_full(&*acl, &message.from, caps, |_| async { Ok(vec![]) })
        .await
        .with_context(|| {
            format!(
                "entity management denied for {}: requires {:?}",
                message.from, caps
            )
        })
}

async fn update_stats_entities(ctx: &RpcHandlerCtx<'_>) {
    let names: Vec<String> = ctx.entity_registry.read().await.keys().cloned().collect();
    ctx.stats.write().await.entity_names = names;
}

// ── Generic reply helper ───────────────────────────────────────────────────────

async fn send_rpc_reply(
    incoming: &ma_core::Message,
    ctx: &RpcHandlerCtx<'_>,
    content: Vec<u8>,
) -> Result<()> {
    let sender = Did::try_from(incoming.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", incoming.from))?;

    let mut reply = ma_core::Message::new(
        ctx.our_did,
        &incoming.from,
        MESSAGE_TYPE_RPC_REPLY,
        "application/cbor",
        &content,
        ctx.signing_key,
    )
    .context("failed to build RPC reply")?;
    reply.reply_to = Some(incoming.id.clone());

    let resolver = IpfsGatewayResolver::new(ctx.kubo_rpc_url.to_string());
    match ctx
        .endpoint
        .outbox(&resolver, &sender.base_id(), RPC_PROTOCOL_ID)
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

async fn send_rpc_ok_reply(
    incoming: &ma_core::Message,
    ctx: &RpcHandlerCtx<'_>,
) -> Result<()> {
    let mut payload = Vec::new();
    ciborium::ser::into_writer(&CborValue::Text(":ok".to_string()), &mut payload)
        .context("failed to encode :ok reply")?;
    send_rpc_reply(incoming, ctx, payload).await
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
