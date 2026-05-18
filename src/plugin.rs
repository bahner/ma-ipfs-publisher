//! Extism-based Wasm plugin wrapper for entity dispatch.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context, Result};
use extism::{host_fn, Function, Manifest, Plugin, UserData, Wasm, PTR};
use ma_core::{cat_bytes, ipfs_add};
use tokio::sync::RwLock;
use tracing::{debug, warn};

use crate::entity::{CastInput, EntityNode, PluginKind, ReplyRequest, SendEnvelope};

// ── Host functions ────────────────────────────────────────────────────────────

// `ma_send` host function exposed to plugins (namespace `extism:host/user`).
//
// The plugin passes a CBOR-encoded `SendEnvelope`.  The host deserialises it
// and pushes it onto the per-call queue; the runtime drains the queue after
// each dispatch.
host_fn!(ma_send_fn(user_data: Vec<SendEnvelope>; input: Vec<u8>) -> Vec<u8> {
    let envelope: SendEnvelope = from_cbor_bytes(&input)?;
    user_data.get()?.lock().unwrap().push(envelope);
    Ok(Vec::new())
});

// `ma_reply` host function: convenience wrapper around `ma_send`.
//
// Plugin passes a CBOR-encoded `ReplyRequest { msg, content }`.  The runtime
// fills in `to` (= msg.from), `reply_to` (= msg.id), and `content_type`
// automatically — plugin only provides the reply body.
host_fn!(ma_reply_fn(user_data: Vec<SendEnvelope>; input: Vec<u8>) -> Vec<u8> {
    let req: ReplyRequest = from_cbor_bytes(&input)?;
    let envelope = SendEnvelope {
        to: req.msg.from,
        content_type: req.content_type,
        content: req.content,
        reply_to: Some(req.msg.id),
    };
    let arc = user_data.get()?;
    arc.lock().unwrap().push(envelope);
    Ok(Vec::new())
});

// Internal state context shared between the `ma_set_state` host function and
// `EntityPlugin`.  Lives inside a `UserData<StateCtx>`.
struct StateCtx {
    /// New state bytes queued for IPFS persistence by the current dispatch.
    pending: Option<Vec<u8>>,
    /// Last successfully persisted snapshot (loaded from IPFS at startup, then
    /// updated by `mark_saved`).  Used for change detection.
    persisted: Option<Vec<u8>>,
    /// `true` when `pending` differs from `persisted` and has not yet been
    /// written to IPFS.
    dirty: bool,
}

impl StateCtx {
    const fn new(persisted: Vec<u8>) -> Self {
        Self {
            pending: None,
            persisted: Some(persisted),
            dirty: false,
        }
    }
}

// `ma_set_state` host function: plugin calls this to queue a new state.
// Sets `dirty` **only** when the bytes actually differ from the last
// persisted snapshot — no-op saves do not pollute the dirty flag.
host_fn!(ma_set_state_fn(user_data: StateCtx; input: Vec<u8>) -> Vec<u8> {
    let arc = user_data.get()?;
    let mut ctx = arc.lock().unwrap();
    if ctx.persisted.as_deref() != Some(input.as_slice()) {
        ctx.pending = Some(input);
        ctx.dirty = true;
        drop(ctx);
    }
    Ok(Vec::new())
});

// ── Dispatch result ─────────────────────────────────────────────────────────

/// Return value from `handle_cast` and `handle_call`.
pub struct DispatchResult {
    /// Outbound messages enqueued via the `ma_send` host function.
    pub envelopes: Vec<SendEnvelope>,
    /// State bytes queued by the plugin via `ma_set_state` host function.
    /// `None` if the plugin did not call `ma_set_state` during this invocation.
    pub pending_state: Option<Vec<u8>>,
}

// ── Registry type alias ───────────────────────────────────────────────────────

/// Thread-safe map from fragment name (e.g. `"fortune"`) to loaded plugin.
pub type EntityRegistry = Arc<RwLock<HashMap<String, Arc<EntityPlugin>>>>;

pub fn new_entity_registry() -> EntityRegistry {
    Arc::new(RwLock::new(HashMap::new()))
}

// ── EntityPlugin ──────────────────────────────────────────────────────────────

pub struct EntityPlugin {
    pub fragment: String,
    pub kind: PluginKind,
    plugin: Mutex<Plugin>,
    /// Queue populated by the `ma_send` host function during a plugin call.
    send_queue: UserData<Vec<SendEnvelope>>,
    /// Shared state context: pending bytes, last-persisted snapshot, dirty flag.
    state: UserData<StateCtx>,
}

// Safety: `extism::Plugin` calls into C (libextism) but the Mutex ensures
// exclusive access and no shared mutable state leaks across thread boundaries.
// extism::Plugin does not implement Send upstream, but our usage is exclusively
// through &mut-guarded Mutex calls, making this sound.
unsafe impl Send for EntityPlugin {}
unsafe impl Sync for EntityPlugin {}

impl EntityPlugin {
    /// Load a plugin from IPFS, initialise it with persisted state (or empty).
    pub async fn load(
        fragment: impl Into<String>,
        node: &EntityNode,
        kubo_url: &str,
    ) -> Result<Self> {
        let fragment = fragment.into();
        let behavior_cid = &node.behavior.cid;
        let kind = PluginKind::from_kind_str(&node.kind);

        debug!(fragment = %fragment, cid = %behavior_cid, kind = ?kind, "loading entity plugin");

        // 1. Fetch Wasm bytes from IPFS.
        let wasm_bytes = cat_bytes(kubo_url, behavior_cid)
            .await
            .with_context(|| format!("fetching wasm for {fragment} from {behavior_cid}"))?;

        // 2. For stateful plugins: fetch persisted state so StateCtx has the
        //    correct baseline and init() can restore it.  Stateless plugins
        //    have no state; module-level code handles any one-time setup.
        let init_state: Vec<u8> = if kind == PluginKind::Stateful {
            match &node.state {
                Some(link) => cat_bytes(kubo_url, &link.cid).await.unwrap_or_default(),
                None => Vec::new(),
            }
        } else {
            Vec::new()
        };

        // 3. Build extism manifest; register host functions.
        //    All plugins get ma_send and ma_reply (outbound messaging).
        //    Stateful plugins additionally get ma_set_state (persistence).
        let send_queue: UserData<Vec<SendEnvelope>> = UserData::new(Vec::new());
        let state: UserData<StateCtx> = UserData::new(StateCtx::new(init_state.clone()));
        let ma_send = Function::new("ma_send", [PTR], [PTR], send_queue.clone(), ma_send_fn);
        let ma_reply = Function::new("ma_reply", [PTR], [PTR], send_queue.clone(), ma_reply_fn);
        let mut host_fns = vec![ma_send, ma_reply];
        if kind == PluginKind::Stateful {
            host_fns.push(Function::new(
                "ma_set_state",
                [PTR],
                [PTR],
                state.clone(),
                ma_set_state_fn,
            ));
        }
        let manifest = Manifest::new([Wasm::data(wasm_bytes)]);
        let wasi = PluginKind::wasi_from_kind_str(&node.kind);
        let mut plugin = tokio::task::block_in_place(|| Plugin::new(&manifest, host_fns, wasi))
            .map_err(|e| anyhow!("failed to create extism plugin for {fragment}: {e}"))?;

        // 4. Stateful only: call init() with the persisted state snapshot.
        if kind == PluginKind::Stateful {
            tokio::task::block_in_place(|| {
                plugin
                    .call::<&[u8], Vec<u8>>("init", init_state.as_slice())
                    .map_err(|e| anyhow!("init() failed for {fragment}: {e}"))
            })?;
        }

        Ok(Self {
            fragment,
            kind,
            plugin: Mutex::new(plugin),
            send_queue,
            state,
        })
    }

    /// Dispatch to the `handle_cast` export (stateless — no state threading).
    pub fn handle_cast(&self, input: &CastInput) -> Result<DispatchResult> {
        self.dispatch_to("handle_cast", input)
    }

    /// Dispatch to the `handle_call` export (stateful).
    ///
    /// `dirty` is only set if the plugin calls `ma_set_state` with bytes that
    /// actually differ from the last persisted snapshot.
    pub fn handle_call(&self, input: &CastInput) -> Result<DispatchResult> {
        self.dispatch_to("handle_call", input)
    }

    /// Encode `input`, invoke the named WASM export, drain both host-function
    /// queues, and return a `DispatchResult`.
    fn dispatch_to(&self, export: &str, input: &CastInput) -> Result<DispatchResult> {
        let mut input_bytes = Vec::new();
        ciborium::ser::into_writer(input, &mut input_bytes)
            .map_err(|e| anyhow!("failed to CBOR-encode CastInput: {e}"))?;

        tokio::task::block_in_place(|| {
            let mut plugin = self
                .plugin
                .lock()
                .map_err(|e| anyhow!("plugin mutex poisoned: {e}"))?;
            plugin
                .call::<&[u8], Vec<u8>>(export, input_bytes.as_slice())
                .map_err(|e| anyhow!("{export}() failed for {}: {e}", self.fragment))
        })?;

        let envelopes = self
            .send_queue
            .get()
            .map_err(|e| anyhow!("send_queue error: {e}"))?
            .lock()
            .map_err(|e| anyhow!("send_queue poisoned: {e}"))?
            .drain(..)
            .collect();

        let pending_state = self
            .state
            .get()
            .map_err(|e| anyhow!("state error: {e}"))?
            .lock()
            .map_err(|e| anyhow!("state poisoned: {e}"))?
            .pending
            .take();

        Ok(DispatchResult {
            envelopes,
            pending_state,
        })
    }

    /// Returns `true` if the plugin has state not yet persisted to IPFS.
    #[allow(dead_code)]
    pub fn is_dirty(&self) -> bool {
        self.state
            .get()
            .ok()
            .and_then(|arc| arc.lock().ok().map(|s| s.dirty))
            .unwrap_or(false)
    }

    /// Record a successful IPFS persist: update the persisted snapshot and
    /// clear the dirty flag.  Call this after a successful `ipfs_add`.
    pub fn mark_saved(&self, saved_bytes: Vec<u8>) {
        if let Ok(arc) = self.state.get() {
            if let Ok(mut ctx) = arc.lock() {
                ctx.persisted = Some(saved_bytes);
                ctx.dirty = false;
            }
        }
    }

    /// Tell the plugin to save its state by calling the `save_state` export.
    ///
    /// A well-behaved plugin responds by calling `ma_set_state(bytes)`.  If it
    /// doesn't, `Ok(None)` is returned and `dirty` is unchanged (tough luck).
    /// On IPFS success the `dirty` flag is cleared and the CID is returned.
    #[allow(dead_code)]
    pub async fn trigger_save(&self, kubo_url: &str) -> Result<Option<String>> {
        tokio::task::block_in_place(|| {
            let mut plugin = self
                .plugin
                .lock()
                .map_err(|e| anyhow!("plugin mutex poisoned: {e}"))?;
            plugin
                .call::<&[u8], Vec<u8>>("save_state", b"")
                .map_err(|e| anyhow!("save_state() failed for {}: {e}", self.fragment))
        })?;

        let pending = self
            .state
            .get()
            .map_err(|e| anyhow!("state error: {e}"))?
            .lock()
            .map_err(|e| anyhow!("state poisoned: {e}"))?
            .pending
            .take();

        if let Some(bytes) = pending {
            let cid = ipfs_add(kubo_url, bytes.clone())
                .await
                .map_err(|e| anyhow!("ipfs_add for {} state: {e}", self.fragment))?;
            self.mark_saved(bytes);
            Ok(Some(cid))
        } else {
            warn!(fragment = %self.fragment, "save_state export did not call ma_set_state");
            Ok(None)
        }
    }
}

// ── CBOR helpers ──────────────────────────────────────────────────────────────

fn from_cbor_bytes<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    ciborium::de::from_reader(bytes).map_err(|e| anyhow!("CBOR decode error: {e}"))
}
