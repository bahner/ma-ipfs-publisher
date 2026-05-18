//! Fluent-based i18n for log messages.
//!
//! FTL locale files live on IPFS and are linked either from a standalone
//! `locales_cid` map or from `RuntimeManifest.locales`.
//! Call [`init`] once after Kubo is ready.
//! Falls back to key-name messages if CIDs are absent or Kubo is unreachable,
//! so early startup logs always produce *something*.

use std::collections::HashMap;
use std::sync::OnceLock;

use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

use crate::entity::{IpldLink, RuntimeManifest};
use crate::kubo;

static MESSAGES: OnceLock<HashMap<String, String>> = OnceLock::new();

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
];

/// Initialise i18n by fetching the FTL file for `lang` from IPFS.
///
/// `locales_cid` points to a standalone `{lang -> IPLD link}` map.
/// `root_cid` is used as fallback for older setups that still keep locales in
/// the runtime manifest.
/// Falls back to using key names as messages if fetching fails or CIDs are
/// not yet configured.  Safe to call only once; subsequent calls are no-ops.
pub async fn init(lang: &str, kubo_url: &str, locales_cid: Option<&str>, root_cid: Option<&str>) {
    let messages = load_messages(lang, kubo_url, locales_cid, root_cid).await;

    let _ = MESSAGES.set(messages);
}

/// Return the localised string for `id`.
/// Falls back to `id` itself when [`init`] has not been called or the key is
/// unknown — so callers always get *something* human-readable.
#[must_use]
pub fn t(id: &str) -> String {
    MESSAGES
        .get()
        .and_then(|m| m.get(id))
        .cloned()
        .unwrap_or_else(|| id.to_string())
}

// ── Internals ─────────────────────────────────────────────────────────────────

fn parse_ftl(ftl: &str, lang: &str) -> HashMap<String, String> {
    let Ok(resource) = FluentResource::try_new(ftl.to_string()) else {
        return fallback_messages();
    };
    let langid: LanguageIdentifier = lang
        .parse()
        .unwrap_or_else(|_| "nb".parse().expect("invalid fallback lang"));
    let mut bundle: FluentBundle<FluentResource> = FluentBundle::new(vec![langid]);
    if bundle.add_resource(resource).is_err() {
        return fallback_messages();
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

fn fallback_messages() -> HashMap<String, String> {
    MESSAGE_IDS
        .iter()
        .map(|&id| (id.to_string(), id.to_string()))
        .collect()
}

fn pick_locale_cid<'a>(lang: &str, locales: &'a HashMap<String, IpldLink>) -> Option<&'a str> {
    locales
        .get(lang)
        .or_else(|| locales.get("en"))
        .or_else(|| locales.values().next())
        .map(|l| l.cid.as_str())
}

async fn load_messages(
    lang: &str,
    kubo_url: &str,
    locales_cid: Option<&str>,
    root_cid: Option<&str>,
) -> HashMap<String, String> {
    if let Some(locales_cid) = locales_cid {
        let locales: HashMap<String, IpldLink> = match kubo::dag_get(kubo_url, locales_cid).await {
            Ok(map) => map,
            Err(_) => return fallback_messages(),
        };

        if let Some(cid) = pick_locale_cid(lang, &locales) {
            return ma_core::cat_bytes(kubo_url, cid).await.map_or_else(
                |_| fallback_messages(),
                |bytes| {
                    String::from_utf8(bytes)
                        .map_or_else(|_| fallback_messages(), |ftl| parse_ftl(&ftl, lang))
                },
            );
        }
    }

    let Some(root_cid) = root_cid else {
        return fallback_messages();
    };

    let manifest: RuntimeManifest = match kubo::dag_get(kubo_url, root_cid).await {
        Ok(m) => m,
        Err(_) => return fallback_messages(),
    };

    let cid = pick_locale_cid(lang, &manifest.locales);

    match cid {
        Some(cid) => ma_core::cat_bytes(kubo_url, cid).await.map_or_else(
            |_| fallback_messages(),
            |bytes| {
                String::from_utf8(bytes)
                    .map_or_else(|_| fallback_messages(), |ftl| parse_ftl(&ftl, lang))
            },
        ),
        None => fallback_messages(),
    }
}
