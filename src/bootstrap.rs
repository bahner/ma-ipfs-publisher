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
    sync::Arc,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::acl::{AclMap, OWNERS_PRINCIPAL};
use ma_core::CapabilityEntry;
use crate::entity::{EntityNode, IpldLink, KindNode, KindTree, PluginKind, RuntimeManifest};
use crate::kubo;
use crate::plugin;

// ── YAML bootstrap schema ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapYaml {
    pub runtime: BootstrapRuntime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapRuntime {
    #[serde(default)]
    pub kinds: BootstrapKindsDict,
    /// Root transport-gate ACL — inline `AclMap` published to IPFS at bootstrap.
    /// Controls who may use the RPC, inbox, and IPFS services.
    /// If absent, the daemon falls back to `--acl-file` (or open access).
    #[serde(default)]
    pub acl: Option<AclMap>,
    /// Entities: bare name → inline entity descriptor.
    /// Bootstrap publishes each as a DAG-CBOR [`EntityNode`] and stores the CID
    /// in the manifest. Keys are bare names (e.g. `"fortune"`), not `#`-prefixed.
    #[serde(default)]
    pub entities: HashMap<String, BootstrapEntity>,
    /// Named ACL library: name → inline `AclMap` published to IPFS at bootstrap.
    /// Reference an ACL by name in an `EntityNode`'s `acl` field.
    #[serde(default)]
    pub acls: HashMap<String, AclMap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapKind {
    #[serde(default)]
    pub api: Vec<String>,
    #[serde(default)]
    pub host_functions: Vec<String>,
    #[serde(default)]
    pub wasi: bool,
}

/// Flat map: protocol ID → kind descriptor.
/// Keys are full protocol ID strings, e.g. `/ma/stateless/python/0.0.1`.
pub type BootstrapKindsDict = BTreeMap<String, BootstrapKind>;

/// Entity entry in the bootstrap YAML — either a bare CID or an inline descriptor.
///
/// ```yaml
/// entities:
///   # pre-published EntityNode — just the CID:
///   rms: QmeB6MAFZ5NTYQgKcPMQ8EimN5rZ6LhbVcirRxTN8t1zoG
///
///   # inline — bootstrap builds and publishes the EntityNode:
///   fortune:
///     kind: /ma/stateless/python/0.0.1
///     behavior: QmaBC...   # Wasm bytes CID
///     acl: open            # optional; empty = deny-all
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BootstrapEntity {
    /// Pre-published [`EntityNode`] — the CID is stored directly in the manifest.
    Cid(String),
    /// Inline descriptor — bootstrap publishes the [`EntityNode`] to IPFS.
    Inline {
        /// Protocol ID of this entity's kind (e.g. `/ma/stateless/python/0.0.1`).
        kind: String,
        /// CID of the Wasm plugin bytes already stored on IPFS.
        behavior: String,
        /// Named ACL reference resolved via `acls.<name>` in the manifest.
        /// Empty string = deny-all (fail-closed).
        #[serde(default)]
        acl: String,
        /// Optional CID of persisted initial state (stateful entities only).
        #[serde(default)]
        state: Option<String>,
    },
}

// ── Result type ───────────────────────────────────────────────────────────────

/// Result of a successful bootstrap run.
#[derive(Debug)]
pub struct BootstrapResult {
    pub root_cid: String,
}

// ── Core bootstrap logic ──────────────────────────────────────────────────────

/// Parse `yaml_path`, publish all IPLD nodes and FTL files to Kubo,
/// and return the resulting CIDs.
pub async fn run_bootstrap(
    yaml_path: &std::path::Path,
    kubo_url: &str,
    runtime_config: BTreeMap<String, serde_yaml::Value>,
    active_lang: &str,
    old_root_cid: Option<&str>,
) -> Result<BootstrapResult> {
    let raw = std::fs::read_to_string(yaml_path)
        .with_context(|| format!("reading bootstrap file: {}", yaml_path.display()))?;
    let yaml: BootstrapYaml = serde_yaml::from_str(&raw).context("parsing bootstrap YAML")?;

    build_manifest(
        &yaml.runtime,
        kubo_url,
        runtime_config,
        active_lang,
        old_root_cid,
    )
    .await
}

/// Build the full IPLD manifest and publish to Kubo. Returns root CID.
pub async fn build_manifest(
    cfg: &BootstrapRuntime,
    kubo_url: &str,
    mut runtime_config: BTreeMap<String, serde_yaml::Value>,
    active_lang: &str,
    old_root_cid: Option<&str>,
) -> Result<BootstrapResult> {
    // 1. Publish kind nodes.
    let mut kinds = KindTree::default();
    for (protocol, bk) in &cfg.kinds {
        let node = KindNode {
            protocol: protocol.clone(),
            api: bk.api.clone(),
            host_functions: bk.host_functions.clone(),
            wasi: bk.wasi,
        };
        let cid = kubo::dag_put(kubo_url, &node)
            .await
            .with_context(|| format!("dag_put kind {protocol}"))?;
        tracing::info!(protocol = %protocol, cid = %cid, "Published kind node");
        kinds.insert_protocol(protocol, IpldLink::new(cid));
    }

    // 2. Build and publish entity nodes.
    let mut entities_map: HashMap<String, IpldLink> = HashMap::new();
    for (name, be) in &cfg.entities {
        let link = match be {
            BootstrapEntity::Cid(cid) => {
                tracing::info!(name = %name, cid = %cid, "Registering pre-published entity");
                IpldLink::new(cid)
            }
            BootstrapEntity::Inline {
                kind,
                behavior,
                acl,
                state,
            } => {
                let node = EntityNode {
                    kind: kind.clone(),
                    behavior: behavior.clone(),
                    acl: acl.clone(),
                    state: state.as_deref().map(IpldLink::new),
                };
                let cid = kubo::dag_put(kubo_url, &node)
                    .await
                    .with_context(|| format!("dag_put entity {name}"))?;
                tracing::info!(name = %name, cid = %cid, "Published entity node");
                IpldLink::new(cid)
            }
        };
        entities_map.insert(name.clone(), link);
    }

    // 3. Publish named ACL nodes.
    let mut acls_map: HashMap<String, IpldLink> = HashMap::new();
    for (name, acl) in &cfg.acls {
        let cid = kubo::dag_put(kubo_url, acl)
            .await
            .with_context(|| format!("dag_put acl {name}"))?;
        tracing::info!(name = %name, cid = %cid, "Published ACL node");
        acls_map.insert(name.clone(), IpldLink::new(cid));
    }

    // 3a. Publish root transport-gate ACL.
    // Always seed OWNERS_PRINCIPAL: ["*"] so the owners group has access out of the box.
    let root_acl_link: Option<IpldLink> = {
        let mut acl_map: AclMap = cfg.acl.clone().unwrap_or_default();
        acl_map
            .entry(OWNERS_PRINCIPAL.to_string())
            .or_insert_with(|| CapabilityEntry::from_caps(["*"]));
        let cid = kubo::dag_put(kubo_url, &acl_map)
            .await
            .context("dag_put root acl")?;
        tracing::info!(cid = %cid, "Published root transport-gate ACL");
        Some(IpldLink::new(cid))
    };

    // 4. Use compiled-in i18n CIDs (generated by `make src/i18n.yaml`).
    let i18n = crate::i18n::bundled_lang_map();
    // Store the active language as a plain BCP-47 string in config so clients
    // can see which language the daemon uses. Changing config.i18n via RPC
    // takes effect immediately (no restart needed).
    if i18n.contains_key(active_lang) {
        runtime_config.insert(
            "i18n".to_string(),
            serde_yaml::Value::String(active_lang.to_string()),
        );
    }
    let root = RuntimeManifest {
        acl: root_acl_link,
        acls: acls_map,
        protocol: "/ma/runtime/0.1.0".to_string(),
        kinds,
        entities: entities_map,
        i18n,
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

/// Export the current runtime manifest as a `BootstrapYaml` YAML string.
///
/// Fetches every linked IPLD node (kinds, entities, named ACLs, root ACL)
/// from Kubo and reconstructs the full bootstrap descriptor so it can be
/// edited and re-bootstrapped with `ma --gen-root-cid`.
pub async fn export_bootstrap_yaml(root_cid: &str, kubo_url: &str) -> Result<String> {
    let manifest: RuntimeManifest = kubo::dag_get(kubo_url, root_cid)
        .await
        .context("fetching root manifest")?;

    // Kinds: fetch each KindNode by CID.
    let mut kinds = BootstrapKindsDict::new();
    for (protocol, kind_link) in manifest.kinds.iter_protocols() {
        let node: KindNode = kubo::dag_get(kubo_url, &kind_link.cid)
            .await
            .with_context(|| format!("fetching kind {protocol}"))?;
        kinds.insert(
            protocol,
            BootstrapKind {
                api: node.api,
                host_functions: node.host_functions,
                wasi: node.wasi,
            },
        );
    }

    // Entities: fetch each EntityNode and reconstruct inline descriptor.
    let mut entities: HashMap<String, BootstrapEntity> = HashMap::new();
    for (name, link) in &manifest.entities {
        let node: EntityNode = kubo::dag_get(kubo_url, &link.cid)
            .await
            .with_context(|| format!("fetching entity {name}"))?;
        entities.insert(
            name.clone(),
            BootstrapEntity::Inline {
                kind: node.kind,
                behavior: node.behavior,
                acl: node.acl,
                state: node.state.map(|s| s.cid),
            },
        );
    }

    // Named ACLs: fetch each AclMap by CID.
    let mut acls: HashMap<String, AclMap> = HashMap::new();
    for (name, link) in &manifest.acls {
        let acl_map: AclMap = kubo::dag_get(kubo_url, &link.cid)
            .await
            .with_context(|| format!("fetching acl {name}"))?;
        acls.insert(name.clone(), acl_map);
    }

    // Root transport-gate ACL.
    let acl: Option<AclMap> = if let Some(ref link) = manifest.acl {
        Some(
            kubo::dag_get(kubo_url, &link.cid)
                .await
                .context("fetching root acl")?,
        )
    } else {
        None
    };

    let yaml = BootstrapYaml {
        runtime: BootstrapRuntime {
            kinds,
            acl,
            entities,
            acls,
        },
    };
    serde_yaml::to_string(&yaml).context("serializing bootstrap YAML")
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

#[cfg(test)]
mod tests {}
