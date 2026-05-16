use anyhow::{anyhow, Context, Result};
use ciborium::Value as CborValue;
use ma_core::{Did, Acl, MESSAGE_TYPE_RPC, MESSAGE_TYPE_RPC_REPLY, SigningKey};
use tracing::{debug, info, warn};

use crate::acl::acl_check;
use crate::status::SharedStats;

pub const RPC_PROTOCOL_ID: &str = "/ma/rpc/0.0.1";
const PING_ATOM: &str = ":ping";
const PONG_ATOM: &str = ":pong";

pub async fn handle_rpc_message(
    message: &ma_core::Message,
    acl: &Acl,
    our_did: &str,
    signing_key: &SigningKey,
    endpoint: &dyn ma_core::MaEndpoint,
    kubo_rpc_url: &str,
    stats: SharedStats,
) -> Result<()> {
    acl_check(acl, &message.from)?;

    if message.message_type != MESSAGE_TYPE_RPC {
        return Err(anyhow!(
            "unsupported RPC content type '{}' on {}",
            message.message_type,
            RPC_PROTOCOL_ID,
        ));
    }

    let term: CborValue = ciborium::de::from_reader(message.content.as_slice())
        .context("invalid CBOR in RPC message")?;

    if !matches!(&term, CborValue::Text(s) if s == PING_ATOM) {
        debug!(from = %message.from, atom = ?term, "{}", crate::i18n::t("unknown-rpc-atom"));
        return Ok(());
    }

    {
        let mut s = stats.write().await;
        s.pings_received += 1;
    }
    info!(from = %message.from, "{}", crate::i18n::t("ping-received"));

    let mut pong_bytes = Vec::new();
    ciborium::ser::into_writer(&CborValue::Text(PONG_ATOM.to_string()), &mut pong_bytes)
        .context("failed to encode :pong")?;

    let sender = Did::try_from(message.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", message.from))?;
    let ping_did_url = format!("did:ma:{}#ping", sender.ipns);

    let mut reply = ma_core::Message::new(
        our_did,
        &ping_did_url,
        MESSAGE_TYPE_RPC_REPLY,
        "application/cbor",
        pong_bytes,
        signing_key,
    )
    .context("failed to build pong message")?;
    reply.reply_to = Some(message.id.clone());

    let resolver = ma_core::IpfsGatewayResolver::new(kubo_rpc_url.to_string());
    match endpoint
        .outbox(&resolver, &sender.base_id(), RPC_PROTOCOL_ID)
        .await
    {
        Ok(mut outbox) => {
            outbox.send(&reply).await.context("pong send failed")?;
            info!(to = %ping_did_url, "{}", crate::i18n::t("pong-sent"));
        }
        Err(err) => {
            warn!(error = %err, to = %ping_did_url, "{}", crate::i18n::t("pong-resolve-failed"));
        }
    }

    Ok(())
}
