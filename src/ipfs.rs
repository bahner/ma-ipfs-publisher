use anyhow::{anyhow, Context, Result};
use ma_core::ipfs::IpfsDidPublisher;
use ma_core::ipfs_add;
use ma_core::{
    validate_ipfs_request, Acl, Did, Inbox, IpfsGatewayResolver, MESSAGE_TYPE_RPC_REPLY,
    ReplayGuard, SigningKey, ValidatedIpfsRequest,
};
use tracing::{info, warn};
use zeroize::Zeroizing;

use crate::acl::acl_check;
use crate::i18n;
use crate::rpc::RPC_PROTOCOL_ID;

/// All state owned by the optional IPFS publisher service.
pub struct IpfsServiceState {
    pub messages: Inbox<ma_core::Message>,
    pub publisher: IpfsDidPublisher,
    pub replay_guard: ReplayGuard,
}

pub struct IpfsHandlerCtx<'a> {
    pub our_did: &'a str,
    pub signing_key: &'a SigningKey,
    pub endpoint: &'a dyn ma_core::MaEndpoint,
    pub kubo_rpc_url: &'a str,
    pub publisher: &'a IpfsDidPublisher,
}

pub async fn do_publish_own_document(
    kubo_url: String,
    doc_cbor: Vec<u8>,
    ipns_secret_key: Vec<u8>,
) -> Result<()> {
    // Wrap in Zeroizing so the key bytes are cleared on return *and* on
    // async cancellation (e.g. if the 2-minute timeout fires and drops
    // the future before the explicit zeroize at the end could run).
    let ipns_secret_key = Zeroizing::new(ipns_secret_key);
    let publisher = IpfsDidPublisher::new(&kubo_url)?;
    publisher.wait_until_ready(10).await?;
    publisher
        .publish_document(&doc_cbor, &ipns_secret_key)
        .await
        .context("kubo publish failed for own DID document")
        .map(|_| ())
}

pub async fn handle_ipfs_message(
    message: &ma_core::Message,
    acl: &Acl,
    ctx: &IpfsHandlerCtx<'_>,
    replay_guard: &mut ReplayGuard,
) -> Result<()> {
    acl_check(acl, &message.from)?;

    let headers = message.headers();
    replay_guard
        .check_and_insert(&headers)
        .context("replay or invalid headers")?;

    let validated = validate_ipfs_request(message).context("invalid /ma/ipfs/0.0.1 request")?;

    match validated {
        ValidatedIpfsRequest::DidDocumentPublish(v) => {
            info!(from = %message.from, id = %message.id, "{}", i18n::t("did-publish-request-received"));
            let key = Zeroizing::new(v.ipns_secret_key.clone());
            let cid = ctx
                .publisher
                .publish_document(&v.document_bytes, &key)
                .await
                .context("kubo DID publish failed")?
                .ok_or_else(|| anyhow!("publisher returned no CID"))?;
            info!(did = %v.document_did.id(), cid = %cid, "{}", i18n::t("document-published"));

            let reply_atom: Vec<ciborium::Value> = vec![
                ciborium::Value::Text(":ok".to_string()),
                ciborium::Value::Text(cid.clone()),
            ];
            let mut reply_bytes = Vec::new();
            ciborium::ser::into_writer(&ciborium::Value::Array(reply_atom), &mut reply_bytes)
                .context("failed to encode ipfs-publish reply")?;

            let sender = Did::try_from(message.from.as_str())
                .with_context(|| format!("invalid sender DID: {}", message.from))?;
            let rpc_did_url = format!("did:ma:{}#rpc", sender.ipns);

            let mut reply = ma_core::Message::new(
                ctx.our_did,
                &rpc_did_url,
                MESSAGE_TYPE_RPC_REPLY,
                "application/cbor",
                reply_bytes,
                ctx.signing_key,
            )
            .context("failed to build ipfs-publish reply")?;
            reply.reply_to = Some(message.id.clone());

            let resolver = IpfsGatewayResolver::new(ctx.kubo_rpc_url.to_string());
            match ctx
                .endpoint
                .outbox(&resolver, &sender.base_id(), RPC_PROTOCOL_ID)
                .await
            {
                Ok(mut outbox) => {
                    outbox
                        .send(&reply)
                        .await
                        .context("ipfs-publish reply send failed")?;
                    info!(to = %rpc_did_url, cid = %cid, "{}", i18n::t("did-publish-cid-reply-sent"));
                }
                Err(err) => {
                    warn!(error = %err, to = %rpc_did_url, "{}", i18n::t("did-publish-resolve-failed"));
                }
            }

            Ok(())
        }
        ValidatedIpfsRequest::Store(v) => handle_ipfs_store(message, &v, ctx).await,
    }
}

async fn handle_ipfs_store(
    orig_message: &ma_core::Message,
    v: &ma_core::ValidatedIpfsStore,
    ctx: &IpfsHandlerCtx<'_>,
) -> Result<()> {
    info!(from = %orig_message.from, id = %orig_message.id, "{}", i18n::t("ipfs-store-request-received"));

    let cid = ipfs_add(ctx.kubo_rpc_url, v.content.clone())
        .await
        .context("ipfs add failed")?;

    info!(cid = %cid, from = %orig_message.from, "{}", i18n::t("ipfs-stored"));

    let reply_atom: Vec<ciborium::Value> = vec![
        ciborium::Value::Text(":ok".to_string()),
        ciborium::Value::Text(cid.clone()),
    ];
    let mut reply_bytes = Vec::new();
    ciborium::ser::into_writer(&ciborium::Value::Array(reply_atom), &mut reply_bytes)
        .context("failed to encode ipfs-store reply")?;

    let sender = Did::try_from(orig_message.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", orig_message.from))?;
    let rpc_did_url = format!("did:ma:{}#rpc", sender.ipns);

    let mut reply = ma_core::Message::new(
        ctx.our_did,
        &rpc_did_url,
        MESSAGE_TYPE_RPC_REPLY,
        "application/cbor",
        reply_bytes,
        ctx.signing_key,
    )
    .context("failed to build ipfs-store reply")?;
    reply.reply_to = Some(orig_message.id.clone());

    let resolver = IpfsGatewayResolver::new(ctx.kubo_rpc_url.to_string());
    match ctx
        .endpoint
        .outbox(&resolver, &sender.base_id(), RPC_PROTOCOL_ID)
        .await
    {
        Ok(mut outbox) => {
            outbox
                .send(&reply)
                .await
                .context("ipfs-store reply send failed")?;
            info!(to = %rpc_did_url, cid = %cid, "{}", i18n::t("ipfs-store-cid-reply-sent"));
        }
        Err(err) => {
            warn!(error = %err, to = %rpc_did_url, "{}", i18n::t("ipfs-store-resolve-failed"));
        }
    }

    Ok(())
}
