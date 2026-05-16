mod i18n;
mod ipfs;
mod acl;
mod rpc;
mod status;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use ma_core::config::{Config, MaArgs, SecretBundle};
use ma_core::ipfs::IpfsDidPublisher;
use ma_core::{ReplayGuard, IPFS_PROTOCOL_ID};
use tracing::{debug, error, info, warn};
use zeroize::Zeroize;

const MA_DEFAULT_SLUG: &str = "ma";

#[derive(Debug, Parser)]
#[command(name = "ma")]
#[command(about = "間 Runtime daemon — RPC + optional IPFS publisher, powered by ma-core")]
struct Cli {
    #[command(flatten)]
    ma: MaArgs,

    /// ACL YAML file. Default: `$XDG_CONFIG_HOME/ma/ma-ipfs-publisher.acl`.
    /// If the default path does not exist the daemon starts with open access (`*`).
    /// Format: `acl: ["*", "did:ma:...", "!did:ma:..."]`
    #[arg(long)]
    acl_file: Option<PathBuf>,

    /// Poll interval in milliseconds.
    #[arg(long, default_value_t = 100)]
    poll_ms: u64,

    /// Language for log messages.
    /// Accepted: nb (default), en.
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
        return Ok(());
    }

    let config = Config::from_args(&cli.ma, MA_DEFAULT_SLUG)?;
    config.init_logging()?;
    i18n::init(&cli.lang);

    let acl = acl::load_acl(cli.acl_file.as_deref())?;

    let ipfs_publisher_enabled = config
        .extra
        .get("ipfs_publisher")
        .and_then(serde_yaml::value::Value::as_bool)
        .unwrap_or(true);

    let secrets = load_secret_bundle(&config)?;

    // ── iroh endpoint (uses iroh_secret_key, separate from IPNS) ──
    let mut endpoint = ma_core::new_ma_endpoint(secrets.iroh_secret_key).await?;

    let rpc_messages = endpoint.service(rpc::RPC_PROTOCOL_ID);

    // ── Build and sign own DID document, publish in background ──
    let ma = endpoint.ma_extension().kind("runtime");
    let our_document = secrets
        .build_document(ma)
        .context("failed to build own DID document")?;
    let our_did = our_document.id.clone();

    let doc_cbor = our_document
        .encode()
        .context("failed to encode own DID document")?;
    let ipns_key = secrets.ipns_secret_key.to_vec();
    let kubo_url_clone = config.kubo_rpc_url.clone();
    let did_for_log = our_did.clone();
    tokio::spawn(async move {
        let result = tokio::time::timeout(
            Duration::from_mins(2),
            ipfs::do_publish_own_document(kubo_url_clone, doc_cbor, ipns_key),
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

    let publisher = IpfsDidPublisher::new(&config.kubo_rpc_url)
        .with_context(|| format!("invalid kubo_rpc_url: {}", config.kubo_rpc_url))?;
    publisher
        .wait_until_ready(10)
        .await
        .context("kubo RPC is not reachable")?;

    // ── Optional IPFS publisher service ──
    let mut ipfs_state = if ipfs_publisher_enabled {
        let messages = endpoint.service(IPFS_PROTOCOL_ID);
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

    info!(
        did = %our_did,
        endpoint_id = %endpoint.id(),
        kubo_rpc_url = %config.kubo_rpc_url,
        status_bind = %cli.status_bind,
        "{}", i18n::t("started")
    );

    // ── Signing key for pong replies ──
    let signing_key = secrets
        .signing_key()
        .context("failed to derive signing key")?;

    // ── Shared status state ──
    let stats = std::sync::Arc::new(tokio::sync::RwLock::new(status::Stats {
        our_did: our_did.clone(),
        endpoint_id: endpoint.id(),
        started_at: status::now_unix_secs(),
        ipfs_publisher_enabled,
        ..Default::default()
    }));

    // ── Status web server ──
    status::spawn_status_server(stats.clone(), cli.status_bind);

    // ── Main event loop ──
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
                        &our_did,
                        &signing_key,
                        &*endpoint,
                        &config.kubo_rpc_url,
                        stats.clone(),
                    ).await {
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
                        ).await {
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

// ── Helpers ───────────────────────────────────────────────────────────────────


