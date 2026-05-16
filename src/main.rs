mod i18n;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use ciborium::Value as CborValue;
use clap::Parser;
use directories::ProjectDirs;
use ma_core::config::{Config, MaArgs, SecretBundle};
use ma_core::ipfs::IpfsDidPublisher;
use ma_core::ipfs_add;
use ma_core::{
    validate_ipfs_request, Acl, Did, IpfsGatewayResolver, ReplayGuard,
    SigningKey, ValidatedIpfsRequest, IPFS_PROTOCOL_ID,
};
use serde_json::json;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use zeroize::{Zeroize, Zeroizing};

const MA_DEFAULT_SLUG: &str = "ma-ipfs-publisher";
const OPEN_ACL_YAML: &str = include_str!("../default.acl");
const RPC_PROTOCOL_ID: &str = "/ma/rpc/0.0.1";
const CONTENT_TYPE_RPC: &str = "application/x-ma-rpc";
const CONTENT_TYPE_RPC_REPLY: &str = "application/x-ma-rpc-reply";
const PING_ATOM: &str = ":ping";
const PONG_ATOM: &str = ":pong";

#[derive(Debug, Parser)]
#[command(name = "ma-ipfs-publisher")]
#[command(about = "Lean /ma/ipfs/0.0.1 + /ma/rpc/0.0.1 daemon powered by ma-core")]
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

/// Shared mutable state for the status endpoint.
#[derive(Default)]
struct Stats {
    our_did: String,
    endpoint_id: String,
    ipfs_requests: u64,
    rpc_requests: u64,
    pings_received: u64,
    started_at: u64,
}

type SharedStats = Arc<RwLock<Stats>>;

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

    let acl = load_acl(cli.acl_file.as_deref())?;

    let secrets = load_secret_bundle(&config)?;

    // ── iroh endpoint (uses iroh_secret_key, separate from IPNS) ──
    let mut endpoint = ma_core::new_ma_endpoint(secrets.iroh_secret_key).await?;

    let ipfs_messages = endpoint.service(IPFS_PROTOCOL_ID);
    let rpc_messages = endpoint.service(RPC_PROTOCOL_ID);

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
            do_publish_own_document(kubo_url_clone, doc_cbor, ipns_key),
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
    let stats = Arc::new(RwLock::new(Stats {
        our_did: our_did.clone(),
        endpoint_id: endpoint.id(),
        started_at: now_unix_secs(),
        ..Default::default()
    }));

    // ── Status web server ──
    spawn_status_server(stats.clone(), cli.status_bind);

    // ── Main event loop ──
    let mut replay_guard = ReplayGuard::default();
    let mut ticker = tokio::time::interval(Duration::from_millis(cli.poll_ms));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let now = now_unix_secs();

                // Drain /ma/rpc/0.0.1
                while let Some(mut message) = rpc_messages.pop(now) {
                    debug!(
                        node = %message.from,
                        protocol = RPC_PROTOCOL_ID,
                        "{}", i18n::t("node-connected")
                    );
                    info!(
                        from = %message.from,
                        to = %message.to,
                        id = %message.id,
                        content_type = %message.content_type,
                        "{}", i18n::t("rpc-message-received")
                    );
                    {
                        let mut s = stats.write().await;
                        s.rpc_requests += 1;
                    }
                    if let Err(err) = handle_rpc_message(
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
                while let Some(mut message) = ipfs_messages.pop(now) {
                    debug!(
                        node = %message.from,
                        protocol = IPFS_PROTOCOL_ID,
                        "{}", i18n::t("node-connected")
                    );
                    debug!(
                        from = %message.from,
                        to = %message.to,
                        id = %message.id,
                        content_type = %message.content_type,
                        content_len = message.content.len(),
                        "{}", i18n::t("received-encrypted-ma-msg")
                    );
                    {
                        let mut s = stats.write().await;
                        s.ipfs_requests += 1;
                    }
                    if let Err(err) = handle_ipfs_message(
                        &message,
                        &acl,
                        &IpfsHandlerCtx {
                            our_did: &our_did,
                            signing_key: &signing_key,
                            endpoint: &*endpoint,
                            kubo_rpc_url: &config.kubo_rpc_url,
                            publisher: &publisher,
                        },
                        &mut replay_guard,
                    ).await {
                        warn!(error = %err, from = %message.from, "{}", i18n::t("ipfs-message-rejected"));
                    }
                    message.content.zeroize();
                    message.signature.zeroize();
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

async fn do_publish_own_document(
    kubo_url: String,
    doc_cbor: Vec<u8>,
    ipns_secret_key: Vec<u8>,
) -> Result<()> {
    // Wrap in Zeroizing so the key bytes are cleared on return *and* on
    // async cancellation (e.g. if the 2-minute timeout fires and drops
    // the future before the explicit zeroize at the end could run).
    let ipns_secret_key = Zeroizing::new(ipns_secret_key);
    let publisher = IpfsDidPublisher::new(&kubo_url)?;
    publisher.wait_until_ready(10).await?;
    publisher
        .publish_document(&doc_cbor, &ipns_secret_key)
        .await
        .context("kubo publish failed for own DID document")
        .map(|_| ())
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

fn spawn_status_server(stats: SharedStats, status_bind: SocketAddr) {
    let status_router = Router::new()
        .route("/", get(handle_index))
        .route("/status.json", get(handle_status_json))
        .with_state(stats);

    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(status_bind)
            .await
            .expect("status server bind failed");
        info!(bind = %status_bind, "{}", i18n::t("status-listening"));
        axum::serve(listener, status_router)
            .await
            .expect("status server failed");
    });
}

// ── RPC handler: respond to :ping with :pong ────────────────────────────────

async fn handle_rpc_message(
    message: &ma_core::Message,
    acl: &Acl,
    our_did: &str,
    signing_key: &SigningKey,
    endpoint: &dyn ma_core::MaEndpoint,
    kubo_rpc_url: &str,
    stats: SharedStats,
) -> Result<()> {
    acl_check(acl, &message.from)?;

    if message.content_type != CONTENT_TYPE_RPC {
        return Err(anyhow!(
            "unsupported RPC content type '{}' on {}",
            message.content_type,
            RPC_PROTOCOL_ID,
        ));
    }

    let term: CborValue = ciborium::de::from_reader(message.content.as_slice())
        .context("invalid CBOR in RPC message")?;

    if !matches!(&term, CborValue::Text(s) if s == PING_ATOM) {
        debug!(from = %message.from, atom = ?term, "{}", i18n::t("unknown-rpc-atom"));
        return Ok(());
    }

    {
        let mut s = stats.write().await;
        s.pings_received += 1;
    }
    info!(from = %message.from, "{}", i18n::t("ping-received"));

    let mut pong_bytes = Vec::new();
    ciborium::ser::into_writer(&CborValue::Text(PONG_ATOM.to_string()), &mut pong_bytes)
        .context("failed to encode :pong")?;

    let sender = Did::try_from(message.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", message.from))?;
    let ping_did_url = format!("did:ma:{}#ping", sender.ipns);

    let mut reply = ma_core::Message::new(
        our_did,
        &ping_did_url,
        CONTENT_TYPE_RPC_REPLY,
        pong_bytes,
        signing_key,
    )
    .context("failed to build pong message")?;
    reply.reply_to = Some(message.id.clone());

    let resolver = ma_core::IpfsGatewayResolver::new(kubo_rpc_url.to_string());
    match endpoint
        .outbox(&resolver, &sender.base_id(), RPC_PROTOCOL_ID)
        .await
    {
        Ok(mut outbox) => {
            outbox.send(&reply).await.context("pong send failed")?;
            info!(to = %ping_did_url, "{}", i18n::t("pong-sent"));
        }
        Err(err) => {
            warn!(error = %err, to = %ping_did_url, "{}", i18n::t("pong-resolve-failed"));
        }
    }

    Ok(())
}

// ── IPFS handler ─────────────────────────────────────────────────────────────

struct IpfsHandlerCtx<'a> {
    our_did: &'a str,
    signing_key: &'a SigningKey,
    endpoint: &'a dyn ma_core::MaEndpoint,
    kubo_rpc_url: &'a str,
    publisher: &'a IpfsDidPublisher,
}

async fn handle_ipfs_message(
    message: &ma_core::Message,
    acl: &Acl,
    ctx: &IpfsHandlerCtx<'_>,
    replay_guard: &mut ReplayGuard,
) -> Result<()> {
    acl_check(acl, &message.from)?;

    let headers = message.headers();
    replay_guard
        .check_and_insert(&headers)
        .context("replay or invalid headers")?;

    let validated = validate_ipfs_request(message).context("invalid /ma/ipfs/0.0.1 request")?;

    match validated {
        ValidatedIpfsRequest::DidDocumentPublish(v) => {
            info!(from = %message.from, id = %message.id, "{}", i18n::t("did-publish-request-received"));
            let key = Zeroizing::new(v.ipns_secret_key.clone());
            let cid = ctx.publisher
                .publish_document(&v.document_bytes, &key)
                .await
                .context("kubo DID publish failed")?
                .ok_or_else(|| anyhow::anyhow!("publisher returned no CID"))?;
            info!(did = %v.document_did.id(), cid = %cid, "{}", i18n::t("document-published"));

            // Send CID reply back to sender's RPC inbox.
            let reply_atom: Vec<ciborium::Value> = vec![
                ciborium::Value::Text(":ok".to_string()),
                ciborium::Value::Text(cid.clone()),
            ];
            let mut reply_bytes = Vec::new();
            ciborium::ser::into_writer(&ciborium::Value::Array(reply_atom), &mut reply_bytes)
                .context("failed to encode ipfs-publish reply")?;

            let sender = Did::try_from(message.from.as_str())
                .with_context(|| format!("invalid sender DID: {}", message.from))?;
            let rpc_did_url = format!("did:ma:{}#rpc", sender.ipns);

            let mut reply = ma_core::Message::new(
                ctx.our_did,
                &rpc_did_url,
                CONTENT_TYPE_RPC_REPLY,
                reply_bytes,
                ctx.signing_key,
            )
            .context("failed to build ipfs-publish reply")?;
            reply.reply_to = Some(message.id.clone());

            let resolver = IpfsGatewayResolver::new(ctx.kubo_rpc_url.to_string());
            match ctx.endpoint
                .outbox(&resolver, &sender.base_id(), RPC_PROTOCOL_ID)
                .await
            {
                Ok(mut outbox) => {
                    outbox.send(&reply).await.context("ipfs-publish reply send failed")?;
                    info!(to = %rpc_did_url, cid = %cid, "{}", i18n::t("did-publish-cid-reply-sent"));
                }
                Err(err) => {
                    warn!(error = %err, to = %rpc_did_url, "{}", i18n::t("did-publish-resolve-failed"));
                }
            }

            Ok(())
        }
        ValidatedIpfsRequest::Store(v) => {
            handle_ipfs_store(message, &v, ctx).await
        }
    }
}

async fn handle_ipfs_store(
    orig_message: &ma_core::Message,
    v: &ma_core::ValidatedIpfsStore,
    ctx: &IpfsHandlerCtx<'_>,
) -> Result<()> {
    info!(from = %orig_message.from, id = %orig_message.id, "{}", i18n::t("ipfs-store-request-received"));

    let cid = ipfs_add(ctx.kubo_rpc_url, v.content.clone())
        .await
        .context("ipfs add failed")?;

    info!(cid = %cid, from = %orig_message.from, "{}", i18n::t("ipfs-stored"));

    let reply_atom: Vec<ciborium::Value> = vec![
        ciborium::Value::Text(":ok".to_string()),
        ciborium::Value::Text(cid.clone()),
    ];
    let mut reply_bytes = Vec::new();
    ciborium::ser::into_writer(&ciborium::Value::Array(reply_atom), &mut reply_bytes)
        .context("failed to encode ipfs-store reply")?;

    let sender = Did::try_from(orig_message.from.as_str())
        .with_context(|| format!("invalid sender DID: {}", orig_message.from))?;
    let rpc_did_url = format!("did:ma:{}#rpc", sender.ipns);

    let mut reply = ma_core::Message::new(
        ctx.our_did,
        &rpc_did_url,
        CONTENT_TYPE_RPC_REPLY,
        reply_bytes,
        ctx.signing_key,
    )
    .context("failed to build ipfs-store reply")?;
    reply.reply_to = Some(orig_message.id.clone());

    let resolver = IpfsGatewayResolver::new(ctx.kubo_rpc_url.to_string());
    match ctx.endpoint
        .outbox(&resolver, &sender.base_id(), RPC_PROTOCOL_ID)
        .await
    {
        Ok(mut outbox) => {
            outbox
                .send(&reply)
                .await
                .context("ipfs-store reply send failed")?;
            info!(to = %rpc_did_url, cid = %cid, "{}", i18n::t("ipfs-store-cid-reply-sent"));
        }
        Err(err) => {
            warn!(error = %err, to = %rpc_did_url, "{}", i18n::t("ipfs-store-resolve-failed"));
        }
    }

    Ok(())
}

// ── ACL check ────────────────────────────────────────────────────────────────

fn acl_check(acl: &Acl, from: &str) -> Result<()> {
    let sender = Did::try_from(from).with_context(|| format!("invalid sender DID '{from}'"))?;
    if !acl.is_allowed(&sender.id()) || !acl.is_allowed(&sender.base_id()) {
        return Err(anyhow!("sender denied by ACL: {from}"));
    }
    Ok(())
}

// ── Status web server handlers ────────────────────────────────────────────────

async fn handle_index(State(stats): State<SharedStats>) -> impl IntoResponse {
    let (our_did, endpoint_id, ipfs_requests, rpc_requests, pings_received, uptime) = {
        let s = stats.read().await;
        (
            s.our_did.clone(),
            s.endpoint_id.clone(),
            s.ipfs_requests,
            s.rpc_requests,
            s.pings_received,
            now_unix_secs().saturating_sub(s.started_at),
        )
    };
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>ma-ipfs-publisher</title>
<style>body{{font-family:monospace;max-width:700px;margin:2em auto;background:#111;color:#eee}}
h1{{color:#7cf}}table{{border-collapse:collapse;width:100%}}
td,th{{padding:6px 12px;border:1px solid #333;text-align:left}}
th{{background:#222}}a{{color:#7cf}}</style></head>
<body>
<h1>間 IPFS Publisher</h1>
<table>
<tr><th>Field</th><th>Value</th></tr>
<tr><td>DID</td><td>{our_did}</td></tr>
<tr><td>Endpoint ID (iroh)</td><td>{endpoint_id}</td></tr>
<tr><td>Uptime (seconds)</td><td>{uptime}</td></tr>
<tr><td>IPFS publish requests</td><td>{ipfs_requests}</td></tr>
<tr><td>RPC requests</td><td>{rpc_requests}</td></tr>
<tr><td>Pings received</td><td>{pings_received}</td></tr>
</table>
<p><a href="/status.json">status.json</a></p>
</body></html>"#
    );
    Html(html)
}

async fn handle_status_json(State(stats): State<SharedStats>) -> impl IntoResponse {
    let (our_did, endpoint_id, ipfs_requests, rpc_requests, pings_received, started_at, uptime) = {
        let s = stats.read().await;
        (
            s.our_did.clone(),
            s.endpoint_id.clone(),
            s.ipfs_requests,
            s.rpc_requests,
            s.pings_received,
            s.started_at,
            now_unix_secs().saturating_sub(s.started_at),
        )
    };
    let body = json!({
        "did": our_did,
        "endpoint_id": endpoint_id,
        "uptime_secs": uptime,
        "ipfs_requests": ipfs_requests,
        "rpc_requests": rpc_requests,
        "pings_received": pings_received,
        "started_at": started_at,
    });
    (
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        body.to_string(),
    )
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn default_acl_path() -> Result<PathBuf> {
    ProjectDirs::from("", "ma", "ma")
        .ok_or_else(|| anyhow!("cannot determine XDG base directories"))
        .map(|d| d.config_dir().join(format!("{MA_DEFAULT_SLUG}.acl")))
}

fn load_acl(explicit: Option<&std::path::Path>) -> Result<Acl> {
    match explicit {
        Some(p) => {
            let yaml = std::fs::read_to_string(p)
                .with_context(|| format!("failed to read ACL file {}", p.display()))?;
            info!(path = %p.display(), "ACL loaded from file");
            Acl::new_from_yaml(&yaml).context("invalid ACL YAML")
        }
        None => {
            let default_path = default_acl_path()?;
            if default_path.exists() {
                let yaml = std::fs::read_to_string(&default_path)
                    .with_context(|| format!("failed to read ACL file {}", default_path.display()))?;
                info!(path = %default_path.display(), "ACL loaded from default path");
                Acl::new_from_yaml(&yaml).context("invalid ACL YAML")
            } else {
                info!(path = %default_path.display(), "no ACL file found, starting with open access");
                Acl::new_from_yaml(OPEN_ACL_YAML).context("invalid open ACL")
            }
        }
    }
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}


