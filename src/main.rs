mod acl;
mod bootstrap;
mod entity;
mod i18n;
mod ipfs;
mod kubo;
mod plugin;
mod rpc;
mod status;

use anyhow::{anyhow, Context, Result};
use cid::Cid;
use clap::Parser;
use ma_core::config::{Config, MaArgs, SecretBundle};
use ma_core::ipfs::IpfsDidPublisher;
use ma_core::{ipns_from_secret, Ipld, ReplayGuard, IPFS_PROTOCOL_ID};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use zeroize::Zeroize;

const MA_DEFAULT_SLUG: &str = "ma";

#[derive(Debug, Parser)]
#[command(name = "ma")]
#[command(about = "間 Runtime daemon — RPC + optional IPFS publisher, powered by ma-core")]
struct Cli {
    #[command(flatten)]
    ma: MaArgs,

    /// ACL YAML file. If absent, all callers are permitted.
    #[arg(long)]
    acl_file: Option<PathBuf>,

    /// Publish FTL locales + manifest from YAML and print the resulting root CID, then exit.
    #[arg(long)]
    gen_root_cid: Option<PathBuf>,

    /// WARNING: resets runtime head for this process. If wrong, recover old CID from logs.
    #[arg(long)]
    root_cid: Option<String>,

    /// Re-publish all locale files and update `root_cid` to a manifest
    /// that references the refreshed locale CIDs.
    #[arg(long = "gen-locales-cid", visible_alias = "locales-cid")]
    gen_locales_cid: bool,

    /// Directory containing `.ftl` locale files.
    #[arg(long, default_value = "locales")]
    locales_dir: PathBuf,

    /// Poll interval in milliseconds.
    #[arg(long, default_value_t = 100)]
    poll_ms: u64,

    /// Language for log messages. Accepted: nb (default), en.
    #[arg(long, default_value = "nb", env = "MA_LANG")]
    lang: String,

    /// Status web server bind address.
    #[arg(long, default_value = "127.0.0.1:5003")]
    status_bind: SocketAddr,
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.ma.gen_headless_config {
        Config::gen_headless(&cli.ma, MA_DEFAULT_SLUG)?;
        // Load the freshly written config + bundle, add the runtime_ipns extra
        // key, and re-save so the bundle is complete before first use.
        let config = Config::from_args(&cli.ma, MA_DEFAULT_SLUG)?;
        let mut bundle = load_secret_bundle(&config)?;
        bundle
            .generate_key("runtime_ipns")
            .context("failed to generate 'runtime_ipns' key")?;
        let passphrase = config
            .secret_bundle_passphrase
            .as_deref()
            .ok_or_else(|| anyhow!("secret_bundle_passphrase missing after gen_headless"))?;
        let bundle_path = config.effective_secret_bundle()?;
        bundle
            .save(&bundle_path, passphrase)
            .context("failed to re-save bundle with 'runtime_ipns' key")?;
        return Ok(());
    }

    let config = Config::from_args(&cli.ma, MA_DEFAULT_SLUG)?;
    config.init_logging()?;

    // ── gen-root-cid: publish bootstrap tree + locales to IPFS, print root CID, exit ──
    if let Some(ref yaml_path) = cli.gen_root_cid {
        let publisher = IpfsDidPublisher::new(&config.kubo_rpc_url)
            .with_context(|| format!("invalid kubo_rpc_url: {}", config.kubo_rpc_url))?;
        publisher
            .wait_until_ready(10)
            .await
            .context("kubo RPC is not reachable for bootstrap")?;

        let runtime_config = runtime_manifest_config(&config);
        let result = bootstrap::run_bootstrap(
            yaml_path,
            &config.kubo_rpc_url,
            &cli.locales_dir,
            runtime_config,
            cli.root_cid.as_deref(),
        )
        .await
        .context("bootstrap failed")?;
        println!("{}", result.root_cid);
        return Ok(());
    }

    if let Some(ref cid) = cli.root_cid {
        Cid::try_from(cid.as_str()).with_context(|| format!("invalid --root-cid CID: {cid}"))?;
        info!(root_cid = %cid, "runtime head reset for this session");
    }

    // ── gen-locales-cid: publish locales map only (no manifest rewrite) ───────
    if cli.gen_locales_cid {
        let publisher = IpfsDidPublisher::new(&config.kubo_rpc_url)
            .with_context(|| format!("invalid kubo_rpc_url: {}", config.kubo_rpc_url))?;
        publisher
            .wait_until_ready(10)
            .await
            .context("kubo RPC is not reachable for locales refresh")?;

        let refresh =
            bootstrap::refresh_locales_in_manifest(&config.kubo_rpc_url, &cli.locales_dir)
                .await
                .context("refreshing locales failed")?;
        info!(
            locales_cid = %refresh.locales_cid,
            "Locales refreshed"
        );
        return Ok(());
    }

    let acl = acl::load_acl(cli.acl_file.as_deref())?;

    let ipfs_publisher_enabled = config
        .extra
        .get("ipfs_publisher")
        .and_then(serde_yaml::value::Value::as_bool)
        .unwrap_or(true);

    let secrets = load_secret_bundle(&config)?;

    // ── Runtime IPNS key (separate from the DID-document IPNS key) ───────────
    let runtime_ipns_key: [u8; 32] = secrets
        .get_key("runtime_ipns")
        .copied()
        .ok_or_else(|| anyhow!("secret bundle is missing extra key 'runtime_ipns'"))?;
    let runtime_ipns_id = ipns_from_secret(runtime_ipns_key)
        .context("failed to derive runtime IPNS id from 'runtime_ipns' key")?;

    // ── iroh endpoint ──────────────────────────────────────────────────────────
    let mut endpoint = ma_core::new_ma_endpoint(secrets.iroh_secret_key).await?;
    let rpc_messages = endpoint.service(rpc::RPC_PROTOCOL_ID);
    let ipfs_messages = if ipfs_publisher_enabled {
        Some(endpoint.service(IPFS_PROTOCOL_ID))
    } else {
        None
    };

    // ── Own DID document (ma extension uses protocol + runtime link) ─────────
    let mut root_cid = cli.root_cid.clone();
    let locales_cid = bootstrap::get_locales_cid(&config);
    // Først: bygg ma-extension uten runtime-link for å få DID
    let ma_base = endpoint
        .ma_extension()
        .extra("protocol", Ipld::String("/ma/runtime/0.0.1".to_string()));
    let our_document_base = secrets
        .build_document(ma_base.clone())
        .context("failed to build own DID document (base)")?;
    let _our_did_base = our_document_base.id.clone();

    // ma.runtime skal være en ekte CID-link (bafy...) for direkte DAG-traversering.
    let ma = if let Some(ref rc) = root_cid {
        let runtime_cid =
            Cid::try_from(rc.as_str()).with_context(|| format!("invalid root_cid: {rc}"))?;
        ma_base.clone().extra("runtime", Ipld::Link(runtime_cid))
    } else {
        ma_base.clone()
    };
    let our_document = secrets
        .build_document(ma)
        .context("failed to build own DID document")?;

    let our_did = our_document.id.clone();

    let did_publish_timeout_secs =
        get_u64_setting(&config, "did_document_publishing_timeout_secs", 120);
    let did_publish_lifetime_hours =
        get_u64_setting(&config, "did_document_publishing_lifetime_hours", 8760);
    let did_publish_interval_secs =
        get_u64_setting(&config, "did_document_publishing_interval_secs", 300);

    let doc_cbor = our_document
        .encode()
        .context("failed to encode own DID document")?;
    let ipns_key = secrets.ipns_secret_key.to_vec();
    let kubo_url_clone = config.kubo_rpc_url.clone();
    let did_for_log = our_did.clone();
    tokio::spawn(async move {
        let result = tokio::time::timeout(
            Duration::from_secs(did_publish_timeout_secs),
            ipfs::do_publish_own_document(
                kubo_url_clone,
                doc_cbor,
                ipns_key,
                did_publish_lifetime_hours,
            ),
        )
        .await;
        match result {
            Ok(Ok(())) => info!(did = %did_for_log, "{}", i18n::t("own-did-published")),
            Ok(Err(err)) => {
                error!(did = %did_for_log, error = %format!("{err:#}"), "{}", i18n::t("own-did-publish-failed"));
            }
            Err(_) => {
                error!(did = %did_for_log, "{}", i18n::t("own-did-publish-timeout"));
            }
        }
    });

    // ── Wait for Kubo ──────────────────────────────────────────────────────────
    let publisher = IpfsDidPublisher::new(&config.kubo_rpc_url)
        .with_context(|| format!("invalid kubo_rpc_url: {}", config.kubo_rpc_url))?;
    publisher
        .wait_until_ready(10)
        .await
        .context("kubo RPC is not reachable")?;

    if root_cid.is_none() {
        root_cid =
            ipfs::resolve_runtime_root_cid_by_ipns_id(&config.kubo_rpc_url, &runtime_ipns_id)
                .await
                .context("failed to resolve runtime root CID from IPNS")?;
        if root_cid.is_none() {
            return Err(anyhow!(
                "no runtime root CID found in IPNS; provide --root-cid <cid>"
            ));
        }
    }

    // ── i18n: fetch locale via RuntimeManifest.locales from IPFS ─────────────
    i18n::init(
        &cli.lang,
        &config.kubo_rpc_url,
        locales_cid.as_deref(),
        root_cid.as_deref(),
    )
    .await;

    // ── Optional IPFS publisher service ───────────────────────────────────────
    let mut ipfs_state = if ipfs_publisher_enabled {
        let messages = ipfs_messages.expect("ipfs inbox exists when publisher is enabled");
        info!("IPFS publisher service enabled");
        Some(ipfs::IpfsServiceState {
            messages,
            publisher,
            replay_guard: ReplayGuard::default(),
        })
    } else {
        info!("IPFS publisher service disabled (set ipfs_publisher: true in config to enable)");
        None
    };

    // ── Load entity plugins from IPFS ──────────────────────────────────────────
    let entity_registry = plugin::new_entity_registry();
    if let Some(ref rc) = root_cid {
        let count = bootstrap::load_entities(rc, &config.kubo_rpc_url, &entity_registry).await;
        info!(count = %count, "Entity plugins loaded");
    }

    // ── Load named ACLs into cache ─────────────────────────────────────────────
    let acl_cache = acl::new_acl_cache();
    if let Some(ref rc) = root_cid {
        let manifest: Result<entity::RuntimeManifest, _> =
            kubo::dag_get(&config.kubo_rpc_url, rc).await;
        match manifest {
            Ok(m) => {
                let mut entries = Vec::new();
                // Root verb-ACL library: "acls.<name>"
                for (acl_name, link) in &m.acls {
                    let cache_key = format!("acls.{acl_name}");
                    match acl::load_acl_from_cid(&config.kubo_rpc_url, &link.cid).await {
                        Ok(acl_map) => {
                            info!(key = %cache_key, cid = %link.cid, "Root ACL loaded into cache");
                            entries.push((cache_key, acl_map));
                        }
                        Err(e) => {
                            warn!(key = %cache_key, cid = %link.cid, error = %e, "failed to load root ACL at startup");
                        }
                    }
                }
                for (ns_name, ns_node) in &m.namespaces {
                    // Namespace gate: "<ns>.acl"
                    if let Some(link) = &ns_node.acl {
                        let cache_key = format!("{ns_name}.acl");
                        match acl::load_acl_from_cid(&config.kubo_rpc_url, &link.cid).await {
                            Ok(acl_map) => {
                                info!(key = %cache_key, cid = %link.cid, "Namespace gate ACL loaded into cache");
                                entries.push((cache_key, acl_map));
                            }
                            Err(e) => {
                                warn!(key = %cache_key, cid = %link.cid, error = %e, "failed to load namespace gate ACL at startup");
                            }
                        }
                    }
                    // Namespace verb-ACL library: "<ns>.acls.<name>"
                    for (acl_name, link) in &ns_node.acls {
                        let cache_key = format!("{ns_name}.acls.{acl_name}");
                        match acl::load_acl_from_cid(&config.kubo_rpc_url, &link.cid).await {
                            Ok(acl_map) => {
                                info!(key = %cache_key, cid = %link.cid, "Named ACL loaded into cache");
                                entries.push((cache_key, acl_map));
                            }
                            Err(e) => {
                                warn!(key = %cache_key, cid = %link.cid, error = %e, "failed to load named ACL at startup");
                            }
                        }
                    }
                }
                let mut cache = acl_cache.write().await;
                for (key, acl_map) in entries {
                    cache.insert(key, acl_map);
                }
            }
            Err(e) => {
                warn!(error = %e, "failed to load manifest for ACL cache population");
            }
        }
    }

    // ── Signing key ────────────────────────────────────────────────────────────
    let signing_key = secrets
        .signing_key()
        .context("failed to derive signing key")?;

    // ── Shared stats ───────────────────────────────────────────────────────────
    let entity_names: Vec<String> = entity_registry.read().await.keys().cloned().collect();
    let stats = Arc::new(tokio::sync::RwLock::new(status::Stats {
        our_did: our_did.clone(),
        endpoint_id: endpoint.id(),
        started_at: status::now_unix_secs(),
        ipfs_publisher_enabled,
        entity_names,
        root_cid: root_cid.clone(),
        ..Default::default()
    }));

    let cors_origins = status_cors_allowed_origins(&config);
    status::spawn_status_server(stats.clone(), cli.status_bind, &cors_origins);

    // Periodisk DID-republisering (hver 5. minutt) fra in-memory runtime-head.
    let refresh_kubo_url = config.kubo_rpc_url.clone();
    let refresh_ma_base = ma_base.clone();
    let refresh_runtime_ipns_key = runtime_ipns_key;
    let refresh_bundle_path = config.effective_secret_bundle()?;
    let refresh_passphrase = config
        .secret_bundle_passphrase
        .clone()
        .ok_or_else(|| anyhow!("secret_bundle_passphrase is required for periodic DID publish"))?;
    let refresh_stats = stats.clone();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(did_publish_interval_secs));
        loop {
            ticker.tick().await;
            let Some(latest_root_cid) = refresh_stats.read().await.root_cid.clone() else {
                continue;
            };
            let runtime_cid = match Cid::try_from(latest_root_cid.as_str()) {
                Ok(c) => c,
                Err(err) => {
                    warn!(cid = %latest_root_cid, error = %err, "invalid root_cid for periodic DID publish");
                    continue;
                }
            };
            let bundle = match SecretBundle::load(&refresh_bundle_path, &refresh_passphrase) {
                Ok(b) => b,
                Err(err) => {
                    error!(error = %format!("{err:#}"), "periodic DID load secret bundle failed");
                    continue;
                }
            };
            let ma = refresh_ma_base
                .clone()
                .extra("runtime", Ipld::Link(runtime_cid));
            let document = match bundle.build_document(ma) {
                Ok(doc) => doc,
                Err(err) => {
                    error!(error = %format!("{err:#}"), "periodic DID build failed");
                    continue;
                }
            };
            let doc_cbor = match document.encode() {
                Ok(bytes) => bytes,
                Err(err) => {
                    error!(error = %format!("{err:#}"), "periodic DID encode failed");
                    continue;
                }
            };
            let ipns_key = bundle.ipns_secret_key.to_vec();
            let publish = tokio::time::timeout(
                Duration::from_secs(did_publish_timeout_secs),
                ipfs::do_publish_own_document(
                    refresh_kubo_url.clone(),
                    doc_cbor,
                    ipns_key,
                    did_publish_lifetime_hours,
                ),
            )
            .await;
            match publish {
                Ok(Ok(())) => {
                    info!(runtime_cid = %latest_root_cid, "periodic DID publish succeeded");
                }
                Ok(Err(err)) => {
                    error!(runtime_cid = %latest_root_cid, error = %format!("{err:#}"), "periodic DID publish failed");
                }
                Err(_) => error!(runtime_cid = %latest_root_cid, "periodic DID publish timed out"),
            }

            match tokio::time::timeout(
                Duration::from_secs(did_publish_timeout_secs),
                ipfs::publish_runtime_root_cid(
                    &refresh_kubo_url,
                    &refresh_runtime_ipns_key,
                    &latest_root_cid,
                    did_publish_lifetime_hours,
                ),
            )
            .await
            {
                Ok(Ok(_)) => {
                    info!(runtime_cid = %latest_root_cid, "periodic runtime_ipns publish succeeded");
                }
                Ok(Err(err)) => {
                    error!(runtime_cid = %latest_root_cid, error = %format!("{err:#}"), "periodic runtime_ipns publish failed");
                }
                Err(_) => {
                    error!(runtime_cid = %latest_root_cid, "periodic runtime_ipns publish timed out");
                }
            }
        }
    });

    info!(
        did = %our_did,
        endpoint_id = %endpoint.id(),
        kubo_rpc_url = %config.kubo_rpc_url,
        status_bind = %cli.status_bind,
        "{}", i18n::t("started")
    );

    // ── Main event loop ────────────────────────────────────────────────────────
    let mut ticker = tokio::time::interval(Duration::from_millis(cli.poll_ms));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let now = status::now_unix_secs();

                // Drain /ma/rpc/0.0.1
                while let Some(mut message) = rpc_messages.pop(now) {
                    debug!(
                        node = %message.from,
                        protocol = rpc::RPC_PROTOCOL_ID,
                        "{}", i18n::t("node-connected")
                    );
                    info!(
                        from = %message.from,
                        to = %message.to,
                        id = %message.id,
                        message_type = %message.message_type,
                        "{}", i18n::t("rpc-message-received")
                    );
                    {
                        let mut s = stats.write().await;
                        s.rpc_requests += 1;
                    }
                    if let Err(err) = rpc::handle_rpc_message(
                        &message,
                        &acl,
                        &rpc::RpcHandlerCtx {
                            our_did: &our_did,
                            signing_key: &signing_key,
                            endpoint: &*endpoint,
                            kubo_rpc_url: &config.kubo_rpc_url,
                            entity_registry: entity_registry.clone(),
                            stats: stats.clone(),
                            acl_cache: acl_cache.clone(),
                            root_acl: &acl,
                        },
                    )
                    .await
                    {
                        warn!(error = %err, from = %message.from, "{}", i18n::t("rpc-message-rejected"));
                    }
                    message.content.zeroize();
                    message.signature.zeroize();
                }

                // Drain /ma/ipfs/0.0.1
                if let Some(ref mut ipfs) = ipfs_state {
                    while let Some(mut message) = ipfs.messages.pop(now) {
                        debug!(
                            node = %message.from,
                            protocol = IPFS_PROTOCOL_ID,
                            "{}", i18n::t("node-connected")
                        );
                        debug!(
                            from = %message.from,
                            to = %message.to,
                            id = %message.id,
                            message_type = %message.message_type,
                            content_len = message.content.len(),
                            "{}", i18n::t("received-encrypted-ma-msg")
                        );
                        {
                            let mut s = stats.write().await;
                            s.ipfs_requests += 1;
                        }
                        if let Err(err) = ipfs::handle_ipfs_message(
                            &message,
                            &acl,
                            &ipfs::IpfsHandlerCtx {
                                our_did: &our_did,
                                signing_key: &signing_key,
                                endpoint: &*endpoint,
                                kubo_rpc_url: &config.kubo_rpc_url,
                                publisher: &ipfs.publisher,
                            },
                            &mut ipfs.replay_guard,
                        )
                        .await
                        {
                            warn!(error = %err, from = %message.from, "{}", i18n::t("ipfs-message-rejected"));
                        }
                        message.content.zeroize();
                        message.signature.zeroize();
                    }
                }
            }
            signal = tokio::signal::ctrl_c() => {
                if let Err(err) = signal {
                    error!(error = %err, "{}", i18n::t("ctrlc-handler-failed"));
                }
                info!("{}", i18n::t("shutdown-requested"));

                // ── Persist entity states before exit ─────────────────────────
                let active_root_cid = stats.read().await.root_cid.clone();
                if let Some(ref rc) = active_root_cid {
                    let count = entity_registry.read().await.len();
                    if count > 0 {
                        info!(count = %count, "{}", i18n::t("entity-states-saving"));
                        match bootstrap::save_all_entity_states(
                            rc,
                            &config.kubo_rpc_url,
                            &entity_registry,
                        )
                        .await
                        {
                            Ok(new_cid) => {
                                stats.write().await.root_cid = Some(new_cid.clone());
                                info!(cid = %new_cid, "{}", i18n::t("entity-states-saved"));
                            }
                            Err(e) => warn!(error = %e, "Failed to save entity states"),
                        }
                    }

                    let latest_root_cid = stats.read().await.root_cid.clone().unwrap_or_else(|| rc.clone());
                    match tokio::time::timeout(
                        Duration::from_secs(did_publish_timeout_secs),
                        ipfs::publish_runtime_root_cid(
                            &config.kubo_rpc_url,
                            &runtime_ipns_key,
                            &latest_root_cid,
                            did_publish_lifetime_hours,
                        ),
                    )
                    .await
                    {
                        Ok(Ok(_)) => info!(runtime_cid = %latest_root_cid, "shutdown runtime_ipns publish succeeded"),
                        Ok(Err(err)) => error!(runtime_cid = %latest_root_cid, error = %format!("{err:#}"), "shutdown runtime_ipns publish failed"),
                        Err(_) => error!(runtime_cid = %latest_root_cid, "shutdown runtime_ipns publish timed out"),
                    }
                }

                break;
            }
        }
    }

    info!("{}", i18n::t("closing-endpoint"));
    endpoint.close().await;
    info!("{}", i18n::t("shutdown-complete"));
    Ok(())
}

fn load_secret_bundle(config: &Config) -> Result<SecretBundle> {
    let passphrase = config
        .secret_bundle_passphrase
        .as_deref()
        .ok_or_else(|| anyhow!("secret_bundle_passphrase is required (env or config)"))?;
    let bundle_path = config.effective_secret_bundle()?;
    SecretBundle::load(&bundle_path, passphrase).with_context(|| {
        format!(
            "failed to load secret bundle from {}",
            bundle_path.display()
        )
    })
}

fn get_u64_setting(config: &Config, key: &str, default: u64) -> u64 {
    config
        .extra
        .get(key)
        .and_then(serde_yaml::Value::as_u64)
        .unwrap_or(default)
}

fn runtime_manifest_config(
    config: &Config,
) -> std::collections::BTreeMap<String, serde_json::Value> {
    let mut out = std::collections::BTreeMap::new();

    out.insert(
        "slug".to_string(),
        serde_json::Value::String(MA_DEFAULT_SLUG.to_string()),
    );
    out.insert(
        "did_resolver_positive_ttl_secs".to_string(),
        serde_json::Value::from(get_u64_setting(
            config,
            "did_resolver_positive_ttl_secs",
            60,
        )),
    );
    out.insert(
        "did_resolver_negative_ttl_secs".to_string(),
        serde_json::Value::from(get_u64_setting(
            config,
            "did_resolver_negative_ttl_secs",
            10,
        )),
    );
    out.insert(
        "did_document_publishing_interval_secs".to_string(),
        serde_json::Value::from(get_u64_setting(
            config,
            "did_document_publishing_interval_secs",
            300,
        )),
    );
    out.insert(
        "did_document_publishing_timeout_secs".to_string(),
        serde_json::Value::from(get_u64_setting(
            config,
            "did_document_publishing_timeout_secs",
            120,
        )),
    );
    out.insert(
        "did_document_publishing_lifetime_hours".to_string(),
        serde_json::Value::from(get_u64_setting(
            config,
            "did_document_publishing_lifetime_hours",
            8760,
        )),
    );
    out.insert(
        "ipns_publish_lifetime_hours".to_string(),
        serde_json::Value::from(get_u64_setting(config, "ipns_publish_lifetime_hours", 8760)),
    );
    out.insert(
        "ipns_publish_resolve".to_string(),
        serde_json::Value::from(
            config
                .extra
                .get("ipns_publish_resolve")
                .and_then(serde_yaml::Value::as_bool)
                .unwrap_or(false),
        ),
    );
    out.insert(
        "ipns_publish_allow_offline".to_string(),
        serde_json::Value::from(
            config
                .extra
                .get("ipns_publish_allow_offline")
                .and_then(serde_yaml::Value::as_bool)
                .unwrap_or(true),
        ),
    );
    out.insert(
        "status_cors_allowed_origins".to_string(),
        serde_json::Value::Array(
            status_cors_allowed_origins(config)
                .into_iter()
                .map(serde_json::Value::String)
                .collect(),
        ),
    );

    out
}

fn status_cors_allowed_origins(config: &Config) -> Vec<String> {
    const DEFAULTS: [&str; 4] = [
        "http://localhost:8000",
        "http://127.0.0.1:8000",
        "https://xn--nf5a.bahner.com",
        "https://間.bahner.com",
    ];

    if let Some(val) = config.extra.get("status_cors_allowed_origins") {
        if let Some(items) = val.as_sequence() {
            let origins: Vec<String> = items
                .iter()
                .filter_map(serde_yaml::Value::as_str)
                .map(ToString::to_string)
                .collect();
            if !origins.is_empty() {
                return origins;
            }
        }
        if let Some(s) = val.as_str() {
            let origins: Vec<String> = s
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToString::to_string)
                .collect();
            if !origins.is_empty() {
                return origins;
            }
        }
    }

    DEFAULTS.iter().map(ToString::to_string).collect()
}
