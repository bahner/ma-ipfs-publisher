//! Intra-runtime message types, plugin I/O types, and IPLD schema types for
//! the entity dispatch system.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Nested IPLD kinds tree — enables IPLD path traversal like:
///   `ipfs dag get .../kinds/ma/stateless/python/0.0.1`
///
/// Protocol IDs such as `/ma/stateless/python/0.0.1` are stored by stripping
/// the leading `/` and splitting on `/`, forming a tree:
///   `kinds.ma.stateless.python["0.0.1"]` = `{"/": "bafy..."}`
///
/// Use [`KindTree::insert_protocol`], [`KindTree::get_protocol`], etc.
/// for protocol-ID-based access. Serialises as a plain nested JSON/CBOR object.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KindTree(BTreeMap<String, KindTreeNode>);

/// A node in the nested [`KindTree`].
///
/// Either a leaf (IPLD link to a published [`KindNode`]) or a branch
/// (inner map continuing the path traversal).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KindTreeNode {
    /// Leaf: IPLD link to a published [`KindNode`].
    Leaf(IpldLink),
    /// Branch: inner map continuing the path traversal.
    Branch(BTreeMap<String, Self>),
}

impl KindTree {
    /// Insert a kind by full protocol ID (e.g. `/ma/stateless/python/0.0.1`).
    pub fn insert_protocol(&mut self, protocol: &str, link: IpldLink) {
        let segments: Vec<&str> = protocol.trim_start_matches('/').split('/').collect();
        kind_tree_insert(&mut self.0, &segments, link);
    }

    /// Look up an IPLD link by full protocol ID.
    pub fn get_protocol(&self, protocol: &str) -> Option<&IpldLink> {
        let segments: Vec<&str> = protocol.trim_start_matches('/').split('/').collect();
        kind_tree_get(&self.0, &segments)
    }

    /// Remove a kind by full protocol ID. Returns `true` when something was removed.
    pub fn remove_protocol(&mut self, protocol: &str) -> bool {
        let segments: Vec<&str> = protocol.trim_start_matches('/').split('/').collect();
        kind_tree_remove(&mut self.0, &segments)
    }

    /// Collect all protocol IDs stored in this tree (each prefixed with `/`).
    pub fn protocol_ids(&self) -> Vec<String> {
        let mut ids = Vec::new();
        kind_tree_collect_ids(&self.0, "", &mut ids);
        ids
    }

    /// Iterate over all `(protocol_id, link)` pairs in the tree.
    pub fn iter_protocols(&self) -> impl Iterator<Item = (String, &IpldLink)> {
        self.protocol_ids()
            .into_iter()
            .filter_map(|id| self.get_protocol(&id).map(|link| (id, link)))
    }
}

fn kind_tree_insert(map: &mut BTreeMap<String, KindTreeNode>, segments: &[&str], link: IpldLink) {
    match segments {
        [] => {}
        [key] => {
            map.insert((*key).to_string(), KindTreeNode::Leaf(link));
        }
        [key, rest @ ..] => {
            let entry = map
                .entry((*key).to_string())
                .or_insert_with(|| KindTreeNode::Branch(BTreeMap::new()));
            if let KindTreeNode::Branch(inner) = entry {
                kind_tree_insert(inner, rest, link);
            }
        }
    }
}

fn kind_tree_get<'a>(
    map: &'a BTreeMap<String, KindTreeNode>,
    segments: &[&str],
) -> Option<&'a IpldLink> {
    match segments {
        [] => None,
        [key] => match map.get(*key)? {
            KindTreeNode::Leaf(link) => Some(link),
            KindTreeNode::Branch(_) => None,
        },
        [key, rest @ ..] => match map.get(*key)? {
            KindTreeNode::Branch(inner) => kind_tree_get(inner, rest),
            KindTreeNode::Leaf(_) => None,
        },
    }
}

fn kind_tree_remove(map: &mut BTreeMap<String, KindTreeNode>, segments: &[&str]) -> bool {
    match segments {
        [] => false,
        [key] => map.remove(*key).is_some(),
        [key, rest @ ..] => {
            if let Some(KindTreeNode::Branch(inner)) = map.get_mut(*key) {
                let removed = kind_tree_remove(inner, rest);
                if inner.is_empty() {
                    map.remove(*key);
                }
                removed
            } else {
                false
            }
        }
    }
}

fn kind_tree_collect_ids(
    map: &BTreeMap<String, KindTreeNode>,
    prefix: &str,
    ids: &mut Vec<String>,
) {
    for (key, node) in map {
        let path = format!("{prefix}/{key}");
        match node {
            KindTreeNode::Leaf(_) => ids.push(path),
            KindTreeNode::Branch(inner) => kind_tree_collect_ids(inner, &path, ids),
        }
    }
}

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
    #[serde(with = "serde_bytes")]
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

/// Context returned by the `ma_ctx` host function.
///
/// Plugin calls `ma_ctx()` once (typically at module load) and caches the
/// result.  Runtime populates all fields from the entity registry — plugins
/// cannot forge or modify them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityCtx {
    /// Full DID-URL of this entity, e.g. `did:ma:<runtime>#fortune`.
    #[serde(rename = "self")]
    pub self_did: String,
    /// Bare fragment without `#` prefix.
    pub fragment: String,
    /// Protocol ID of the kind, e.g. `/ma/root/0.0.1`.
    pub kind: String,
    /// Fragment of the parent entity.  `None` for `#root`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Lifecycle state managed by the runtime.
    pub lifecycle: Lifecycle,
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
    #[serde(with = "serde_bytes")]
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
    /// How the runtime executes plugin bytes for this kind.
    /// Defaults to [`Evaluator::Extism`] when absent in serialised form.
    #[serde(default)]
    pub evaluator: Evaluator,
    /// Kind attributes. Required keys: `stateful` (bool), `wasi` (bool).
    /// `stateful: true` means the runtime must call `init()`, pass persisted
    /// state in and persist new state out after each call.  Never inferred
    /// from the `api` list — the explicit attribute is the source of truth.
    #[serde(default)]
    pub attributes: BTreeMap<String, serde_json::Value>,
    /// Which caller entity kinds are allowed to create instances of this kind.
    /// `"create *"` means any kind may create instances.
    /// Absence of `"public: true"` in attributes AND an empty allow list means
    /// only the parent entity may create instances of this kind.
    #[serde(default)]
    pub allow: Vec<String>,
}

impl KindNode {
    /// Whether plugins of this kind require WASI system-call support.
    pub fn wasi(&self) -> bool {
        self.attributes
            .get("wasi")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    }

    /// Whether plugins of this kind are stateful.
    /// Read from the explicit `stateful` attribute — never derived from the `api` list.
    /// Stateful plugins have state loaded before `init()` / `handle_call` and
    /// persisted afterwards; stateless plugins have no such lifecycle.
    pub fn plugin_kind(&self) -> PluginKind {
        if self
            .attributes
            .get("stateful")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            PluginKind::Stateful
        } else {
            PluginKind::Stateless
        }
    }
}

/// In-memory registry of loaded [`KindNode`]s.  Single source of truth for all
/// kind attributes at runtime.  Populated at bootstrap and updated on kind upsert.
pub type KindRegistry = Arc<RwLock<HashMap<String, Arc<KindNode>>>>;

/// Create a new, empty [`KindRegistry`].
pub fn new_kind_registry() -> KindRegistry {
    Arc::new(RwLock::new(HashMap::new()))
}

/// Entity fragment names reserved by the runtime system.
/// These names cannot be used as entity names.
pub const RESERVED_ENTITY_NAMES: &[&str] = &["root", "acl", "scheduler", "logger", "runtime"];

/// Lifecycle state of an entity, persisted in [`EntityNode`] and passed to
/// plugins via [`EntityCtx`].
///
/// | State | Meaning |
/// |-------|---------|
/// | `new` | Created but `init()` not yet completed |
/// | `running` | `init()` completed OK — normal dispatch |
/// | `error` | `init()` failed on restart — plugin still dispatchable for `:debug`/`:dump` |
/// | `stopped` | Clean shutdown via runtime signal |
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Lifecycle {
    New,
    #[default]
    Running,
    Error,
    Stopped,
}

impl std::fmt::Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lifecycle::New => write!(f, "new"),
            Lifecycle::Running => write!(f, "running"),
            Lifecycle::Error => write!(f, "error"),
            Lifecycle::Stopped => write!(f, "stopped"),
        }
    }
}

/// How the runtime executes the plugin bytes for a [`KindNode`].
///
/// Stored in `KindNode.evaluator`; defaults to [`Extism`](Evaluator::Extism).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Evaluator {
    #[default]
    Extism,
    Native,
    Bash,
    Lua,
}

/// IPLD node representing a single entity.
///
/// Access is controlled by the entity-level ACL.  WASI support is derived
/// from the `kind` protocol string at plugin-load time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityNode {
    pub kind: String,
    /// IPLD link to the Wasm plugin bytes stored on IPFS.
    /// Absent for native entities (e.g. `#scheduler`) that have no Wasm.
    /// Stored as `{"/": "bafy…"}` so Kubo's recursive pin follows it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behaviour: Option<IpldLink>,
    /// Entity verb-ACL — name string resolved via `acls.<name>` in the root
    /// manifest (e.g. `"fortune"`). Cached under `"acls.<name>"` at startup.
    /// Empty string means deny-all (fail-closed).
    #[serde(default)]
    pub acl: String,
    /// IPLD link to persisted state (optional).
    /// Omitted when absent, which is the expected shape for stateless entities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IpldLink>,
    /// DID-URL of this entity's parent in the entity tree.
    /// Absent for `#root` (tree root). Used to derive ACL and cascade delete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Human-readable label for display purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Lifecycle state: `New` until first successful `init()`, then `Running`.
    #[serde(default)]
    pub lifecycle: Lifecycle,
}

/// Root IPLD node for this runtime.
/// Stored as CID in `config.yaml` and published into the DID document under
/// `ma.runtime`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    pub i18n: HashMap<String, IpldLink>,
    #[serde(default)]
    pub config: BTreeMap<String, serde_yaml::Value>,
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
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
    pub reply_to: Option<String>,
}

/// A request to create a new entity, queued by the `ma_create_entity` host function.
/// Processed by the runtime after the plugin dispatch returns.
#[derive(Debug, Clone)]
pub struct CreateEntityRequest {
    /// Pre-generated nanoid-style fragment for the new entity (8 chars, URL-safe).
    pub fragment: String,
    /// Protocol ID of the kind to instantiate (e.g. `/ma/stateless/ping/0.0.1`).
    pub kind_protocol: String,
    /// IPFS CID of the Wasm plugin bytes — becomes `EntityNode.behaviour`.
    pub behaviour_cid: String,
    /// Fragment of the creating (parent) entity.
    pub parent: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{EntityNode, IpldLink, KindTree};
    use std::collections::HashMap;

    #[test]
    fn kind_tree_nested_serialization() {
        let mut tree = KindTree::default();
        tree.insert_protocol("/ma/stateless/python/0.0.1", IpldLink::new("bafyAAA"));
        tree.insert_protocol("/ma/stateful/python/0.0.1", IpldLink::new("bafyBBB"));

        let val = serde_json::to_value(&tree).expect("serialize kind tree");
        // Verify nested structure accessible via path segments.
        assert_eq!(
            val["ma"]["stateless"]["python"]["0.0.1"]["/"], "bafyAAA",
            "stateless python leaf"
        );
        assert_eq!(
            val["ma"]["stateful"]["python"]["0.0.1"]["/"], "bafyBBB",
            "stateful python leaf"
        );
    }

    #[test]
    fn kind_tree_protocol_ids_roundtrip() {
        let mut tree = KindTree::default();
        tree.insert_protocol("/ma/stateless/python/0.0.1", IpldLink::new("bafyAAA"));
        tree.insert_protocol("/ma/stateful/python/0.0.1", IpldLink::new("bafyBBB"));

        let mut ids = tree.protocol_ids();
        ids.sort();
        assert_eq!(
            ids,
            vec!["/ma/stateful/python/0.0.1", "/ma/stateless/python/0.0.1"]
        );
    }

    #[test]
    fn kind_tree_remove_prunes_empty_branches() {
        let mut tree = KindTree::default();
        tree.insert_protocol("/ma/stateless/python/0.0.1", IpldLink::new("bafyAAA"));
        assert!(tree.remove_protocol("/ma/stateless/python/0.0.1"));
        assert!(
            tree.protocol_ids().is_empty(),
            "tree should be empty after remove"
        );
    }

    #[test]
    fn serializing_entity_without_state_omits_state_field() {
        let node = EntityNode {
            kind: "/ma/stateless/python/0.0.1".to_string(),
            behaviour: Some(IpldLink {
                cid: "bafybehaviour".to_string(),
            }),
            acl: String::new(),
            state: None,
            parent: None,
            label: None,
            lifecycle: Lifecycle::default(),
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
            behaviour: Some(IpldLink {
                cid: "bafybehaviour".to_string(),
            }),
            acl: String::new(),
            state: None,
            parent: None,
            label: None,
            lifecycle: Lifecycle::default(),
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
            "behaviour": {"/": "bafybehaviour"},
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
            "behaviour": {"/": "bafybehaviour"},
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
    fn deserializing_entity_accepts_missing_behaviour_field() {
        let raw = r#"{
            "kind": "/ma/scheduler/0.0.1",
            "acl": "open"
        }"#;

        let node: EntityNode = serde_json::from_str(raw).expect("deserialize entity node");
        assert!(
            node.behaviour.is_none(),
            "missing behaviour should deserialize as None (native entity)"
        );
    }
}
