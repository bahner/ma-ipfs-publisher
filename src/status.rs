use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Default)]
pub struct Stats {
    pub our_did: String,
    pub endpoint_id: String,
    pub ipfs_requests: u64,
    pub rpc_requests: u64,
    pub pings_received: u64,
    pub started_at: u64,
    pub ipfs_publisher_enabled: bool,
}

pub type SharedStats = Arc<RwLock<Stats>>;

pub fn spawn_status_server(stats: SharedStats, status_bind: SocketAddr) {
    let status_router = Router::new()
        .route("/", get(handle_index))
        .route("/status.json", get(handle_status_json))
        .with_state(stats);

    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(status_bind)
            .await
            .expect("status server bind failed");
        info!(bind = %status_bind, "{}", crate::i18n::t("status-listening"));
        axum::serve(listener, status_router)
            .await
            .expect("status server failed");
    });
}

pub fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

async fn handle_index(State(stats): State<SharedStats>) -> impl IntoResponse {
    let (our_did, endpoint_id, ipfs_requests, rpc_requests, pings_received, uptime, ipfs_enabled) = {
        let s = stats.read().await;
        (
            s.our_did.clone(),
            s.endpoint_id.clone(),
            s.ipfs_requests,
            s.rpc_requests,
            s.pings_received,
            now_unix_secs().saturating_sub(s.started_at),
            s.ipfs_publisher_enabled,
        )
    };
    let ipfs_status = if ipfs_enabled { "enabled" } else { "disabled" };
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>間 Runtime</title>
<style>body{{font-family:monospace;max-width:700px;margin:2em auto;background:#111;color:#eee}}
h1{{color:#7cf}}table{{border-collapse:collapse;width:100%}}
td,th{{padding:6px 12px;border:1px solid #333;text-align:left}}
th{{background:#222}}a{{color:#7cf}}</style></head>
<body>
<h1>間 Runtime</h1>
<table>
<tr><th>Field</th><th>Value</th></tr>
<tr><td>DID</td><td>{our_did}</td></tr>
<tr><td>Endpoint ID (iroh)</td><td>{endpoint_id}</td></tr>
<tr><td>Uptime (seconds)</td><td>{uptime}</td></tr>
<tr><td>IPFS publisher</td><td>{ipfs_status}</td></tr>
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
    let (our_did, endpoint_id, ipfs_requests, rpc_requests, pings_received, started_at, uptime, ipfs_enabled) = {
        let s = stats.read().await;
        (
            s.our_did.clone(),
            s.endpoint_id.clone(),
            s.ipfs_requests,
            s.rpc_requests,
            s.pings_received,
            s.started_at,
            now_unix_secs().saturating_sub(s.started_at),
            s.ipfs_publisher_enabled,
        )
    };
    let body = json!({
        "did": our_did,
        "endpoint_id": endpoint_id,
        "uptime_secs": uptime,
        "ipfs_publisher": ipfs_enabled,
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
