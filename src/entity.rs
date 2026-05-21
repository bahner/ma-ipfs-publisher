//! Intra-runtime message types, plugin I/O types, and IPLD schema types for
//! the entity dispatch system.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

pub type KindTree = BTreeMap<String, BTreeMap<String, KindRef>>;

// ── IPLD link ─────────────────────────────────────────────────────────────────

/// An IPLD DAG link serialised as `{"/": "bafy..."}`.
///
/// Kubo's `dag/put` endpoint with `input-codec=dag-json` recognises this as a
/// proper IPLD link, enabling `ipfs dag get` traversal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpldLink {
    #[serde(rename = "/")]
    pub cid: String,
}

impl IpldLink {
    pub fn new(cid: impl Into<String>) -> Self {
        Self { cid: cid.into() }
    }
}

// ── Local (intra-runtime) message ─────────────────────────────────────────────

#[allow(dead_code)]
pub const LOCAL_RPC_CONTENT_TYPE: &str = "application/x-ma-local-rpc";

/// An intra-runtime message.  Follows the same schema as `ma_core::Message`
/// but `from` and `to` may be bare fragments (`#fragment`) or full DIDs, and
/// no signature is required — the runtime is the trusted authority for local
/// dispatch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalMessage {
    pub id: String,
    pub from: String,
    pub to: String,
    /// Unix nanoepoch.
    pub created_at: u64,
    /// Expiry as Unix nanoepoch.
    pub expires: u64,
    pub reply_to: Option<String>,
    pub content_type: String,
    /// CBOR-encoded payload (verb atom or `[":verb", args…]` array).
    pub content: Vec<u8>,
}

// ── Plugin context and I/O ────────────────────────────────────────────────────

/// Which ABI a plugin kind implements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginKind {
    /// Stateless — exports `handle_cast`.  No state is passed in or out.
    /// Replies are sent via the `ma_send` host function.
    Stateless,
    /// Stateful — exports `handle_call`.  Runtime passes current state in;
    /// plugin returns new state bytes.  Replies via `ma_send` host function.
    Stateful,
}

impl PluginKind {
    /// Derive kind from the `EntityNode.kind` protocol string
    /// (e.g. `/ma/stateful/python/0.0.1`).
    pub fn from_kind_str(s: &str) -> Self {
        if s.contains("stateful") {
            Self::Stateful
        } else {
            Self::Stateless
        }
    }

    /// Derive whether a plugin requires WASI system-call support from the kind
    /// protocol string.  Python-compiled and explicit WASI plugins return `true`;
    /// native Rust extism plugins return `false`.
    pub fn wasi_from_kind_str(s: &str) -> bool {
        s.contains("python") || s.contains("wasi")
    }
}

/// Context injected into every plugin call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCtx {
    /// Full DID-URL of this entity, e.g. `did:ma:<runtime>#fortune`.
    #[serde(rename = "self")]
    pub self_did: String,
}

/// Input passed (CBOR-encoded) to both `handle_cast` (stateless) and
/// `handle_call` (stateful) exports.
///
/// State is **not** included here — stateful plugins own their state
/// internally and receive it once via `init(state_bytes)`.  They persist it
/// by calling the `ma_set_state` host function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastInput {
    pub msg: LocalMessage,
    pub ctx: PluginCtx,
}

/// Input for the `ma_reply` host function.
///
/// Plugin passes the original `LocalMessage` back alongside reply content;
/// the runtime fills in `to` and `reply_to` automatically from `msg`.
/// `content_type` is the MIME type of the reply body (e.g. `text/plain`,
/// `application/cbor`).  The wire `message_type` is set automatically by the
/// runtime based on `reply_to` being present.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyRequest {
    /// The original incoming message — used to determine routing.
    pub msg: LocalMessage,
    /// MIME type of the reply body.
    pub content_type: String,
    /// Serialised reply body bytes.
    pub content: Vec<u8>,
}

// ── IPLD schema types ─────────────────────────────────────────────────────────

/// IPLD node representing a kind (Wasm ABI contract).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindNode {
    pub protocol: String,
    /// Wasm exports this kind must provide.
    pub api: Vec<String>,
    /// Host functions the runtime makes available to plugins of this kind.
    /// Principle of least privilege: only register what the kind actually needs.
    pub host_functions: Vec<String>,
    /// Whether plugins of this kind require WASI system-call support.
    /// Set `true` for Python/WASI-compiled plugins; `false` for native Rust extism plugins.
    #[serde(default)]
    pub wasi: bool,
}

/// Kind reference stored in nested dict layout:
/// kinds.<family>.<implementation>.
///
/// New manifests store a plain IPLD link. Older manifests stored an object
/// with duplicted metadata and a nested `link` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KindRef {
    Link(IpldLink),
    Legacy { link: IpldLink },
}

impl KindRef {
    #[allow(dead_code)]
    pub const fn link(&self) -> &IpldLink {
        match self {
            Self::Link(link) | Self::Legacy { link } => link,
        }
    }
}

/// A namespace within the runtime manifest. Owned by a single DID.
///
/// ## Key conventions
///
/// | Key pattern | Meaning |
/// |-------------|---------|
/// | `acl` | Namespace gate — IPLD link to an `AclMap` document |
/// | `acls` | Named ACL library — flat map of name → IPLD link |
/// | anything else | Free IPLD sub-tree (blob, list, nested object) |
///
/// Entities are **not** stored inside namespace nodes. All entities live in
/// the flat `entities` map at the manifest root, identified by a globally
/// unique bare name. DID fragment = entity name: `did:ma:<ipns>#fortune`.
///
/// Blob values are stored as `{"/": "bafy…"}` in `extra`.  Nested structure
/// is supported lazily: `alice.prosjekt.mappe.ting` resolves `extra["prosjekt"]`
/// to a root CID and traverses the remaining path via `ipfs dag resolve`.
/// Nested structures are managed externally; the namespace only stores the
/// root link.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NamespaceNode {
    /// Optional human-readable name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Namespace gate — IPLD link to an `AclMap` document.
    /// Required for any blob access; absent = deny all blob operations.
    /// Cached at startup under key `"<ns>.acl"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acl: Option<IpldLink>,
    /// Named verb-ACL library — flat map of name → IPLD link to `AclMap`.
    /// Each entry is cached under `"<ns>.acls.<name>"`.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub acls: HashMap<String, IpldLink>,
    /// Free-form IPLD sub-trees for organisational use.
    /// Values must be IPLD-compatible JSON; CID links are automatically
    /// followed by `ipfs dag get`.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// IPLD node representing a single entity.
///
/// Access is controlled by the entity-level ACL.  WASI support is derived
/// from the `kind` protocol string at plugin-load time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityNode {
    pub kind: String,
    /// IPLD link to the Wasm plugin bytes stored on IPFS.
    pub behavior: IpldLink,
    /// Entity verb-ACL — name string resolved via `acls.<name>` in the root
    /// manifest (e.g. `"fortune"`). Cached under `"acls.<name>"` at startup.
    /// Empty string means deny-all (fail-closed).
    #[serde(default)]
    pub acl: String,
    /// IPLD link to persisted state (optional).
    /// Omitted when absent, which is the expected shape for stateless entities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IpldLink>,
}

/// Root IPLD node for this runtime.
/// Stored as CID in `config.yaml` and published into the DID document under
/// `ma.runtime`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeManifest {
    /// Transport-level ACL — IPLD link to an ACL document.
    /// Loaded once at startup. When absent, falls back to the YAML-based
    /// ACL supplied via `--acl-file`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acl: Option<IpldLink>,
    /// Root-level verb-ACL library — name → IPLD link to `AclMap`.
    /// Used by root entities (`entity.acl` resolves here).
    /// Each entry cached at startup under `"acls.<name>"`.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub acls: HashMap<String, IpldLink>,
    /// Runtime protocol identifier (e.g. `"/ma/runtime/0.1.0"`).
    #[serde(default)]
    pub protocol: String,
    /// Global kinds registry. Shared across all namespaces.
    #[serde(default)]
    pub kinds: KindTree,
    /// Global entity registry — bare name → IPLD link to [`EntityNode`].
    ///
    /// Keys are globally unique entity names (e.g. `"fortune"`, `"rms"`).
    /// No `#` prefix. The corresponding DID fragment equals the key:
    /// `did:ma:<ipns>#fortune` ↔ `entities["fortune"]`.
    pub entities: HashMap<String, IpldLink>,
    #[serde(default)]
    pub lang: HashMap<String, IpldLink>,
    #[serde(default)]
    pub config: BTreeMap<String, serde_json::Value>,
    /// Namespace nodes keyed by handle (e.g. `"owner"`, `"alice"`).
    ///
    /// Reserved handles: `acl`, `acls`, `protocol`, `kinds`, `entities`, `lang`, `config`.
    #[serde(flatten)]
    pub namespaces: HashMap<String, NamespaceNode>,
}

impl RuntimeManifest {
    #[allow(dead_code)]
    pub fn kind_link(&self, protocol: &str) -> Option<&IpldLink> {
        let (_, family, implementation, _) = split_kind_protocol(protocol)?;
        Some(self.kinds.get(family)?.get(implementation)?.link())
    }
}

#[allow(dead_code)]
fn split_kind_protocol(protocol: &str) -> Option<(&str, &str, &str, &str)> {
    // Expected: /ma/<family>/<implementation>/<version>
    let parts: Vec<&str> = protocol.trim_matches('/').split('/').collect();
    if parts.len() == 4 && parts[0] == "ma" {
        Some((parts[0], parts[1], parts[2], parts[3]))
    } else {
        None
    }
}

// ── Plugin host-function I/O ──────────────────────────────────────────────────

/// Outbound message queued by a plugin via the `ma_send` host function.
///
/// `to`       — recipient DID (or DID-URL).
/// `reply_to` — if set, marks this as a reply to the given message ID; the
///              runtime will use `MESSAGE_TYPE_RPC_REPLY` for the wire message.
/// `content`  — raw payload bytes (e.g. CBOR-encoded RPC atom or JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEnvelope {
    pub to: String,
    pub content_type: String,
    pub content: Vec<u8>,
    pub reply_to: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{EntityNode, IpldLink, PluginKind};

    #[test]
    fn serializing_entity_without_state_omits_state_field() {
        let node = EntityNode {
            kind: "/ma/stateless/python/0.0.1".to_string(),
            behavior: IpldLink::new("bafybehavior"),
            acl: String::new(),
            state: None,
        };

        let value = serde_json::to_value(&node).expect("serialize entity node");
        assert!(
            value.get("state").is_none(),
            "state must be omitted when None"
        );
    }

    #[test]
    fn serializing_entity_always_includes_acl_field() {
        let node = EntityNode {
            kind: "/ma/stateless/python/0.0.1".to_string(),
            behavior: IpldLink::new("bafybehavior"),
            acl: String::new(),
            state: None,
        };
        let value = serde_json::to_value(&node).expect("serialize entity node");
        assert!(
            value.get("acl").is_some(),
            "acl must always be present in serialized form"
        );
    }

    #[test]
    fn deserializing_entity_accepts_missing_state_field() {
        let raw = r#"{
            "kind": "/ma/stateless/python/0.0.1",
            "behavior": {"/": "bafybehavior"},
            "acl": ""
        }"#;

        let node: EntityNode = serde_json::from_str(raw).expect("deserialize entity node");
        assert!(
            node.state.is_none(),
            "missing state should deserialize as None"
        );
    }

    #[test]
    fn deserializing_entity_accepts_null_state_field() {
        let raw = r#"{
            "kind": "/ma/stateless/python/0.0.1",
            "behavior": {"/": "bafybehavior"},
            "acl": "",
            "state": null
        }"#;

        let node: EntityNode = serde_json::from_str(raw).expect("deserialize entity node");
        assert!(
            node.state.is_none(),
            "null state should deserialize as None"
        );
    }

    #[test]
    fn wasi_derived_from_python_kind_string() {
        assert!(PluginKind::wasi_from_kind_str("/ma/stateless/python/0.0.1"));
        assert!(PluginKind::wasi_from_kind_str("/ma/stateful/wasi/0.0.1"));
        assert!(!PluginKind::wasi_from_kind_str("/ma/stateless/rust/0.0.1"));
        assert!(!PluginKind::wasi_from_kind_str("/ma/stateful/rust/0.0.1"));
    }
}
