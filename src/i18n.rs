//! Fluent-based i18n for log messages and RPC replies.
//!
//! FTL lang files live on IPFS and are linked either from a standalone
//! `lang_cid` map or from `RuntimeManifest.lang`.
//! Call [`init`] once after Kubo is ready.
//! Compile-time bundled FTL is used as fallback so startup logs always produce
//! *something* human-readable even before IPFS is available.

use std::collections::HashMap;
use std::sync::OnceLock;

use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

use crate::entity::{IpldLink, RuntimeManifest};
use crate::kubo;

/// All messages keyed by `(lang_code, message_id)`.
static MESSAGES: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();
static RUNTIME_LANG: OnceLock<String> = OnceLock::new();

const BUNDLED_EN_FTL: &str = include_str!("../lang/en.ftl");
const BUNDLED_NB_FTL: &str = include_str!("../lang/nb.ftl");

/// All known message IDs — must match keys in both FTL files.
const MESSAGE_IDS: &[&str] = &[
    // Startup / shutdown
    "own-did-published",
    "own-did-publish-failed",
    "own-did-publish-timeout",
    "started",
    "shutdown-requested",
    "closing-endpoint",
    "shutdown-complete",
    // Infrastructure
    "status-listening",
    "rpc-message-received",
    "rpc-message-rejected",
    "ipfs-message-rejected",
    "ctrlc-handler-failed",
    "node-connected",
    "received-encrypted-ma-msg",
    // RPC
    "unknown-rpc-atom",
    "rpc-reply-sent",
    "ping-received",
    // IPFS publisher
    "did-publish-request-received",
    "document-published",
    "did-publish-cid-reply-sent",
    "did-publish-resolve-failed",
    "ipfs-store-request-received",
    "ipfs-stored",
    "ipfs-store-cid-reply-sent",
    "ipfs-store-resolve-failed",
    // Entity dispatch
    "bootstrap-complete",
    "entity-loaded",
    "entity-load-failed",
    "entity-not-found",
    "entity-dispatched",
    "entity-replied",
    "root-create-entity",
    "root-list-entities",
    "root-delete-entity",
    "root-entity-updated",
    "entity-created",
    "entity-deleted",
    "entity-states-saving",
    "entity-state-saving",
    "entity-state-saved",
    "entity-state-empty",
    "entity-states-saved",
    // i18n itself
    "ftl-loaded",
    // First-run auto-init
    "no-config-found",
    "initialising-new-identity",
    "generated-headless-config",
    // Ownership / claim
    "runtime-claimed",
    "runtime-claim-persisted",
    "runtime-already-claimed",
    // Client-facing RPC error replies
    "refuse-delete-root",
    "no-root-acl",
    "namespace-not-found",
    "no-ns-gate-acl",
];

/// Initialise i18n by fetching ALL available FTL bundles from IPFS.
///
/// `lang` is the runtime's own language; used as the default for [`t`].
/// `lang_cid` points to a standalone `{lang -> IPLD link}` map.
/// `root_cid` is a fallback for older manifests that embed lang inline.
/// Compile-time bundled FTL is always loaded first, so the runtime is
/// fully operational even when IPFS is unreachable.
/// Safe to call only once; subsequent calls are no-ops.
pub async fn init(lang: &str, kubo_url: &str, lang_cid: Option<&str>, root_cid: Option<&str>) {
    let _ = RUNTIME_LANG.set(lang.to_string());
    let messages = load_all_messages(kubo_url, lang_cid, root_cid).await;
    let _ = MESSAGES.set(messages);
}

/// Return the localised string for `id` in the runtime's own language.
/// Falls back to `id` itself when [`init`] has not been called or the key is
/// unknown — so callers always get *something* human-readable.
#[must_use]
pub fn t(id: &str) -> String {
    t_lang(runtime_lang(), id)
}

/// Return the localised string for `id` in `lang`.
/// Falls back to "en" if `lang` is not loaded, then to the key name.
#[must_use]
pub fn t_lang(lang: &str, id: &str) -> String {
    MESSAGES
        .get()
        .and_then(|all| {
            all.get(lang)
                .or_else(|| all.get("en"))
                .and_then(|m| m.get(id))
        })
        .cloned()
        .unwrap_or_else(|| id.to_string())
}

/// The runtime's own language code (set by [`init`]; defaults to "en").
#[must_use]
pub fn runtime_lang() -> &'static str {
    RUNTIME_LANG.get().map(String::as_str).unwrap_or("en")
}

/// Returns `true` if messages for `lang` have been loaded.
#[must_use]
pub fn has_lang(lang: &str) -> bool {
    MESSAGES.get().is_some_and(|m| m.contains_key(lang))
}

// ── Internals ─────────────────────────────────────────────────────────────────

fn parse_ftl(ftl: &str, lang: &str) -> HashMap<String, String> {
    let Ok(resource) = FluentResource::try_new(ftl.to_string()) else {
        return key_name_fallback();
    };
    let langid: LanguageIdentifier = lang
        .parse()
        .unwrap_or_else(|_| "en".parse().expect("invalid fallback lang"));
    let mut bundle: FluentBundle<FluentResource> = FluentBundle::new(vec![langid]);
    if bundle.add_resource(resource).is_err() {
        return key_name_fallback();
    }

    let mut map = HashMap::new();
    for &id in MESSAGE_IDS {
        if let Some(msg) = bundle.get_message(id) {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let value = bundle.format_pattern(pattern, None, &mut errors);
                map.insert(id.to_string(), value.into_owned());
            } else {
                map.insert(id.to_string(), id.to_string());
            }
        } else {
            map.insert(id.to_string(), id.to_string());
        }
    }
    map
}

fn key_name_fallback() -> HashMap<String, String> {
    MESSAGE_IDS
        .iter()
        .map(|&id| (id.to_string(), id.to_string()))
        .collect()
}

fn bundled_messages() -> HashMap<String, HashMap<String, String>> {
    let mut all = HashMap::new();
    all.insert("en".to_string(), parse_ftl(BUNDLED_EN_FTL, "en"));
    all.insert("nb".to_string(), parse_ftl(BUNDLED_NB_FTL, "nb"));
    all
}

async fn fetch_lang_map(
    kubo_url: &str,
    lang_cid: Option<&str>,
    root_cid: Option<&str>,
) -> HashMap<String, IpldLink> {
    if let Some(cid) = lang_cid {
        if let Ok(map) = kubo::dag_get(kubo_url, cid).await {
            return map;
        }
    }
    if let Some(cid) = root_cid {
        if let Ok(manifest) = kubo::dag_get::<RuntimeManifest>(kubo_url, cid).await {
            return manifest.lang;
        }
    }
    HashMap::new()
}

async fn load_all_messages(
    kubo_url: &str,
    lang_cid: Option<&str>,
    root_cid: Option<&str>,
) -> HashMap<String, HashMap<String, String>> {
    let mut all = bundled_messages();

    let lang_map = fetch_lang_map(kubo_url, lang_cid, root_cid).await;
    for (code, link) in &lang_map {
        if let Ok(bytes) = ma_core::cat_bytes(kubo_url, &link.cid).await {
            if let Ok(ftl) = String::from_utf8(bytes) {
                all.insert(code.clone(), parse_ftl(&ftl, code));
            }
        }
    }

    all
}

