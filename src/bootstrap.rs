//! Bootstrap: YAML → IPLD manifest + FTL locales → Kubo.
//!
//! Run once before starting the runtime:
//! ```sh
//! ma --gen-root-cid bootstrap.yaml
//! ```
//! CID for the runtime root manifest is written back to `config.yaml`.
//! Subsequent daemon starts load entities and locales from IPFS.

use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
    sync::Arc,
};

use anyhow::{anyhow, Context, Result};
use ma_core::ipfs_add;
use serde::Deserialize;

use crate::entity::{
    EntityNode, IpldLink, KindNode, KindRef, KindTree, PluginKind, RuntimeManifest,
};
use crate::kubo;
use crate::plugin;

pub const LOCALES_CID_KEY: &str = "locales_cid";

// ── YAML bootstrap schema ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct BootstrapYaml {
    pub runtime: BootstrapRuntime,
}

#[derive(Debug, Deserialize)]
pub struct BootstrapRuntime {
    #[serde(default)]
    pub kinds: BootstrapKindsDict,
    /// Entities defined under `entities:` in the YAML.
    /// Keys are bare entity names (e.g. `"fortune"`), not `#`-prefixed.
    #[serde(default)]
    pub entities: HashMap<String, BootstrapEntity>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BootstrapKind {
    pub protocol: String,
    #[serde(default)]
    pub api: Vec<String>,
    #[serde(default)]
    pub host_functions: Vec<String>,
    #[serde(default)]
    pub wasi: bool,
}

pub type BootstrapKindsDict = BTreeMap<String, BTreeMap<String, BootstrapKind>>;

#[derive(Debug, Deserialize)]
pub struct BootstrapEntity {
    pub kind: String,
    pub behavior_cid: String,
    /// Entity-level ACL reference path (e.g. `"ns.acl.write"`).
    /// Empty string means deny-all.
    #[serde(default)]
    pub acl: String,
}

// ── Result type ───────────────────────────────────────────────────────────────

/// CIDs produced by a successful bootstrap run.
#[derive(Debug)]
pub struct BootstrapResult {
    pub root_cid: String,
}

/// CIDs produced by a successful locales refresh run.
#[derive(Debug)]
pub struct LocalesRefreshResult {
    pub locales_cid: String,
}

// ── Core bootstrap logic ──────────────────────────────────────────────────────

/// Parse `yaml_path`, publish all IPLD nodes and FTL files to Kubo,
/// and return the resulting CIDs.
pub async fn run_bootstrap(
    yaml_path: &Path,
    kubo_url: &str,
    locales_dir: &Path,
    runtime_config: BTreeMap<String, serde_json::Value>,
    old_root_cid: Option<&str>,
) -> Result<BootstrapResult> {
    let raw = std::fs::read_to_string(yaml_path)
        .with_context(|| format!("reading bootstrap file: {}", yaml_path.display()))?;
    let yaml: BootstrapYaml = serde_yaml::from_str(&raw).context("parsing bootstrap YAML")?;

    build_manifest(
        &yaml.runtime,
        kubo_url,
        locales_dir,
        runtime_config,
        old_root_cid,
    )
    .await
}

/// Build the full IPLD manifest and publish to Kubo. Returns root CID.
pub async fn build_manifest(
    cfg: &BootstrapRuntime,
    kubo_url: &str,
    locales_dir: &Path,
    runtime_config: BTreeMap<String, serde_json::Value>,
    old_root_cid: Option<&str>,
) -> Result<BootstrapResult> {
    let mut kinds_flat: Vec<BootstrapKind> = Vec::new();

    // 1. Publish kind nodes.
    let mut kinds: KindTree = BTreeMap::new();
    for (family, impls) in &cfg.kinds {
        for (implementation, bk) in impls {
            let parsed = parse_kind_protocol(&bk.protocol)
                .with_context(|| format!("invalid kind protocol: {}", bk.protocol))?;
            if parsed.0 != family || parsed.1 != implementation {
                return Err(anyhow!(
                    "kind key-path {}/{} does not match protocol {}",
                    family,
                    implementation,
                    bk.protocol
                ));
            }

            kinds_flat.push(bk.clone());

            let node = KindNode {
                protocol: bk.protocol.clone(),
                api: bk.api.clone(),
                host_functions: bk.host_functions.clone(),
                wasi: bk.wasi,
            };
            let cid = kubo::dag_put(kubo_url, &node)
                .await
                .with_context(|| format!("dag_put kind {}", bk.protocol))?;
            tracing::info!(protocol = %bk.protocol, cid = %cid, "Published kind node");
            let link = IpldLink::new(cid);
            insert_kind_entry(&mut kinds, family, implementation, KindRef::Link(link));
        }
    }

    // 2. Publish entity nodes.
    let known_kinds: std::collections::HashSet<String> =
        kinds_flat.iter().map(|k| k.protocol.clone()).collect();
    let mut entities_map: HashMap<String, IpldLink> = HashMap::new();
    for (name, be) in &cfg.entities {
        if !known_kinds.is_empty() && !known_kinds.contains(&be.kind) {
            return Err(anyhow!("entity {name} references unknown kind {}", be.kind));
        }
        let entity = build_bootstrap_entity_node(be);
        let cid = kubo::dag_put(kubo_url, &entity)
            .await
            .with_context(|| format!("dag_put entity {name}"))?;
        tracing::info!(name = %name, cid = %cid, "Published entity node");
        entities_map.insert(name.clone(), IpldLink::new(cid));
    }

    // 3. Publish locales and root manifest.
    let locales = publish_locales(locales_dir, kubo_url).await?;
    let _locales_cid = kubo::dag_put(kubo_url, &locales)
        .await
        .context("dag_put locales map")?;
    let root = RuntimeManifest {
        acl: None,
        acls: std::collections::HashMap::new(),
        protocol: "/ma/runtime/0.1.0".to_string(),
        kinds,
        entities: entities_map,
        lang: locales,
        config: runtime_config,
        namespaces: std::collections::HashMap::new(),
    };
    let root_cid = kubo::dag_put(kubo_url, &root)
        .await
        .context("dag_put root manifest")?;
    tracing::info!(root_cid = %root_cid, "Published runtime root manifest");

    // Swap pins atomically: move the recursive pin from the old root to the
    // new one so no intermediate state exists with an unpinned manifest.
    if let Some(old) = old_root_cid {
        if let Err(e) = kubo::pin_update(kubo_url, old, &root_cid).await {
            tracing::warn!(old = %old, new = %root_cid, error = %e, "pin/update failed after bootstrap");
        }
    } else {
        kubo::pin_add(kubo_url, &root_cid)
            .await
            .context("pinning new root manifest")?;
    }

    Ok(BootstrapResult { root_cid })
}

fn insert_kind_entry(tree: &mut KindTree, family: &str, implementation: &str, entry: KindRef) {
    tree.entry(family.to_string())
        .or_default()
        .insert(implementation.to_string(), entry);
}

fn parse_kind_protocol(protocol: &str) -> Result<(&str, &str, &str)> {
    let parts: Vec<&str> = protocol.trim_matches('/').split('/').collect();
    if parts.len() == 4 && parts[0] == "ma" {
        Ok((parts[1], parts[2], parts[3]))
    } else {
        Err(anyhow!(
            "expected /ma/<family>/<implementation>/<version>, got: {protocol}"
        ))
    }
}

fn build_bootstrap_entity_node(be: &BootstrapEntity) -> EntityNode {
    EntityNode {
        kind: be.kind.clone(),
        behavior: IpldLink::new(&be.behavior_cid),
        acl: be.acl.clone(),
        state: None,
    }
}

/// Read standalone locale-map CID from config extra fields.
pub fn get_locales_cid(config: &ma_core::Config) -> Option<String> {
    config
        .extra
        .get(LOCALES_CID_KEY)
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
}

// ── Startup entity loading ────────────────────────────────────────────────────

/// Fetch the `RuntimeManifest` at `root_cid`, load each entity plugin, and
/// insert them into `registry`.  Returns the number of successfully loaded
/// entities.
pub async fn load_entities(
    root_cid: &str,
    kubo_url: &str,
    registry: &plugin::EntityRegistry,
) -> usize {
    let manifest = match kubo::dag_get::<RuntimeManifest>(kubo_url, root_cid).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to fetch runtime manifest {root_cid}: {e}");
            return 0;
        }
    };

    let mut loaded = 0usize;
    for (name, link) in &manifest.entities {
        let node: EntityNode = match kubo::dag_get(kubo_url, &link.cid).await {
            Ok(n) => n,
            Err(e) => {
                tracing::warn!(name = %name, cid = %link.cid, "Failed to fetch entity node: {e}");
                continue;
            }
        };
        match plugin::EntityPlugin::load(name.clone(), &node, kubo_url).await {
            Ok(ep) => {
                tracing::info!(name = %name, "{}", crate::i18n::t("entity-loaded"));
                registry.write().await.insert(name.clone(), Arc::new(ep));
                loaded += 1;
            }
            Err(e) => {
                tracing::warn!(name = %name, error = %e, "{}", crate::i18n::t("entity-load-failed"));
            }
        }
    }
    loaded
}

// ── Graceful shutdown: persist entity states ──────────────────────────────────
/// nodes and root manifest.
/// Logs progress at `info` level with per-entity phases.  Returns the new
/// root CID on success.
pub async fn save_all_entity_states(
    root_cid: &str,
    kubo_url: &str,
    registry: &plugin::EntityRegistry,
) -> Result<String> {
    // Phase 1: fetch current manifest.
    tracing::info!(root_cid = %root_cid, "Fetching current runtime manifest");
    let mut manifest: RuntimeManifest = kubo::dag_get(kubo_url, root_cid)
        .await
        .context("fetching current runtime manifest")?;

    // Snapshot the registry so we don't hold the lock during async IPFS calls.
    let snapshot: Vec<(String, Arc<plugin::EntityPlugin>)> = registry
        .read()
        .await
        .iter()
        .map(|(k, v)| (k.clone(), Arc::clone(v)))
        .collect();

    // Phase 2: persist each entity's state (stateful only).
    for (name, entity) in &snapshot {
        if entity.kind == PluginKind::Stateless {
            continue;
        }

        tracing::info!(name = %name, "{}", crate::i18n::t("entity-state-saving"));

        let state_cid = match entity.trigger_save(kubo_url).await {
            Ok(Some(cid)) => cid,
            Ok(None) => {
                tracing::info!(name = %name, "{}", crate::i18n::t("entity-state-empty"));
                continue;
            }
            Err(e) => {
                tracing::warn!(name = %name, error = %e, "Failed to save entity state");
                continue;
            }
        };
        tracing::info!(name = %name, cid = %state_cid, "{}", crate::i18n::t("entity-state-saved"));

        // Update entity node with new state link.
        let Some(entity_link) = manifest.entities.get(name).cloned() else {
            tracing::warn!(name = %name, "Entity in registry but not in manifest, skipping");
            continue;
        };
        let mut entity_node: EntityNode = match kubo::dag_get(kubo_url, &entity_link.cid).await {
            Ok(n) => n,
            Err(e) => {
                tracing::warn!(name = %name, error = %e, "Failed to fetch entity node for state update");
                continue;
            }
        };
        entity_node.state = Some(IpldLink::new(state_cid));

        match kubo::dag_put(kubo_url, &entity_node).await {
            Ok(new_cid) => {
                tracing::info!(name = %name, cid = %new_cid, "Updated entity node with new state");
                manifest
                    .entities
                    .insert(name.clone(), IpldLink::new(new_cid));
            }
            Err(e) => {
                tracing::warn!(name = %name, error = %e, "Failed to publish updated entity node");
            }
        }
    }

    // Phase 3: publish updated manifest.
    tracing::info!("Publishing updated runtime manifest");
    let new_root_cid = kubo::dag_put(kubo_url, &manifest)
        .await
        .context("dag_put updated manifest")?;

    // Swap pins atomically via pin/update.
    if let Err(e) = kubo::pin_update(kubo_url, root_cid, &new_root_cid).await {
        tracing::warn!(old = %root_cid, new = %new_root_cid, error = %e, "pin/update failed after state save");
    }

    Ok(new_root_cid)
}

/// Re-publish all locale files from `locales_dir` and publish one locale-map CID.
pub async fn refresh_locales_in_manifest(
    kubo_url: &str,
    locales_dir: &Path,
) -> Result<LocalesRefreshResult> {
    let locales = publish_locales(locales_dir, kubo_url).await?;
    let locales_cid = kubo::dag_put(kubo_url, &locales)
        .await
        .context("publishing locales map")?;

    Ok(LocalesRefreshResult { locales_cid })
}

async fn publish_locales(locales_dir: &Path, kubo_url: &str) -> Result<HashMap<String, IpldLink>> {
    let mut locales_map: HashMap<String, IpldLink> = HashMap::new();
    let entries = std::fs::read_dir(locales_dir)
        .with_context(|| format!("reading locales dir {}", locales_dir.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("iterating {}", locales_dir.display()))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("ftl") {
            continue;
        }
        let Some(lang) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        let bytes = std::fs::read(&path)
            .with_context(|| format!("reading locale file {}", path.display()))?;
        let cid = ipfs_add(kubo_url, bytes)
            .await
            .with_context(|| format!("ipfs_add locale {}", path.display()))?;
        tracing::info!(lang = %lang, cid = %cid, "Published locale file");
        locales_map.insert(lang.to_string(), IpldLink::new(cid));
    }

    if locales_map.is_empty() {
        return Err(anyhow!(
            "no .ftl locale files found in {}",
            locales_dir.display()
        ));
    }

    Ok(locales_map)
}

#[cfg(test)]
mod tests {
    use super::{build_bootstrap_entity_node, BootstrapEntity};

    #[test]
    fn bootstrap_stateless_entity_serialization_omits_state_field() {
        let be = BootstrapEntity {
            kind: "/ma/stateless/python/0.0.1".to_string(),
            behavior_cid: "bafybehavior".to_string(),
            acl: String::new(),
        };

        let node = build_bootstrap_entity_node(&be);
        let value = serde_json::to_value(&node).expect("serialize bootstrap entity");

        assert!(
            value.get("state").is_none(),
            "state must be omitted for stateless bootstrap entity"
        );
        assert!(
            value.get("acl").is_some(),
            "acl must be present in serialized form"
        );
    }
}
