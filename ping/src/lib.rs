//! ma-ping-plugin — Extism plugin implementing /ma/ping/0.0.1
//!
//! Receives a `:ping` RPC message and replies with `:pong` to the caller's
//! `#ping` fragment DID-URL (e.g. `did:ma:<ipns>#ping`).
//!
//! # ABI
//!
//! Input:  CBOR-encoded `CastInput`   (same structure as all stateless entities)
//! Output: none (reply sent via `ma_send` host function)
//! Export: `handle_cast`

use extism_pdk::*;
use serde::{Deserialize, Serialize};

// ── Wire types (subset of what the runtime uses) ──────────────────────────────

#[derive(Debug, Deserialize)]
struct LocalMessage {
    id: String,
    from: String,
    #[allow(dead_code)]
    to: String,
    #[allow(dead_code)]
    created_at: u64,
    #[allow(dead_code)]
    #[serde(default)]
    expires: Option<u64>,
    #[allow(dead_code)]
    #[serde(default)]
    reply_to: Option<String>,
    #[allow(dead_code)]
    content_type: String,
    #[allow(dead_code)]
    content: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct PluginCtx {
    #[allow(dead_code)]
    #[serde(rename = "self")]
    self_did: String,
}

#[derive(Debug, Deserialize)]
struct CastInput {
    msg: LocalMessage,
    #[allow(dead_code)]
    ctx: PluginCtx,
}

/// Outbound message envelope passed to `ma_send`.
#[derive(Debug, Serialize, Deserialize)]
struct SendEnvelope {
    to: String,
    content_type: String,
    content: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<String>,
}

// ── Host functions ────────────────────────────────────────────────────────────

#[host_fn]
unsafe extern "ExtismHost" {
    /// Send an arbitrary outbound message.
    /// Input: CBOR-encoded `SendEnvelope`.
    fn ma_send(data: Vec<u8>) -> Vec<u8>;
}

// ── Plugin entrypoint ─────────────────────────────────────────────────────────

#[plugin_fn]
pub fn handle_cast(input: Vec<u8>) -> FnResult<Vec<u8>> {
    // Decode the incoming CastInput.
    let cast: CastInput = cbor_decode(&input)
        .map_err(|e| anyhow::anyhow!("ping: decode input: {e}"))?;

    // Build `:pong` CBOR bytes.
    let pong_bytes = cbor_encode_text(":pong")?;

    // Reply target: caller's `#ping` DID-URL.
    // `msg.from` is the caller's bare DID (unfragmented `:ping` comes from bare DID).
    let to = format!("{}#ping", strip_fragment(&cast.msg.from));

    let envelope = SendEnvelope {
        to,
        content_type: "application/cbor".to_string(),
        content: pong_bytes,
        reply_to: Some(cast.msg.id),
    };

    let envelope_cbor = cbor_encode_value(&envelope)?;
    unsafe { ma_send(envelope_cbor)? };

    Ok(vec![])
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Strip any `#fragment` suffix from a DID string.
fn strip_fragment(did: &str) -> &str {
    did.split('#').next().unwrap_or(did)
}

/// Encode a CBOR text atom (e.g. `":pong"`) to bytes.
fn cbor_encode_text(s: &str) -> FnResult<Vec<u8>> {
    let mut buf = Vec::new();
    ciborium::ser::into_writer(&ciborium::Value::Text(s.to_string()), &mut buf)
        .map_err(|e| anyhow::anyhow!("cbor encode text: {e}"))?;
    Ok(buf)
}

/// Encode a `serde::Serialize` value to CBOR.
fn cbor_encode_value<T: serde::Serialize>(value: &T) -> FnResult<Vec<u8>> {
    let mut buf = Vec::new();
    ciborium::ser::into_writer(value, &mut buf)
        .map_err(|e| anyhow::anyhow!("cbor encode: {e}"))?;
    Ok(buf)
}

/// Decode CBOR bytes to a `serde::DeserializeOwned` type.
fn cbor_decode<T: for<'de> serde::Deserialize<'de>>(bytes: &[u8]) -> Result<T, String> {
    ciborium::de::from_reader(bytes).map_err(|e| format!("CBOR decode: {e}"))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_fragment_bare_did() {
        assert_eq!(strip_fragment("did:ma:k51abc"), "did:ma:k51abc");
    }

    #[test]
    fn strip_fragment_with_fragment() {
        assert_eq!(strip_fragment("did:ma:k51abc#sign"), "did:ma:k51abc");
    }

    #[test]
    fn cbor_encode_text_roundtrip() {
        let bytes = cbor_encode_text(":pong").unwrap();
        let val: ciborium::Value = ciborium::de::from_reader(bytes.as_slice()).unwrap();
        assert_eq!(val, ciborium::Value::Text(":pong".to_string()));
    }

    #[test]
    fn cbor_encode_decode_envelope() {
        let env = SendEnvelope {
            to: "did:ma:abc#ping".to_string(),
            content_type: "application/cbor".to_string(),
            content: vec![1, 2, 3],
            reply_to: Some("msg-id".to_string()),
        };
        let bytes = cbor_encode_value(&env).unwrap();
        let decoded: SendEnvelope = cbor_decode(&bytes).unwrap();
        assert_eq!(decoded.to, env.to);
        assert_eq!(decoded.reply_to, env.reply_to);
    }
}
