use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::{header, HeaderValue, Method};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde_json::{json, Value};
use tokio::sync::RwLock;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{info, warn};

#[derive(Default)]
pub struct Stats {
    pub our_did: String,
    pub endpoint_id: String,
    pub ipfs_requests: u64,
    pub rpc_requests: u64,
    pub started_at: u64,
    pub ipfs_publisher_enabled: bool,
    pub entity_names: Vec<String>,
    pub root_cid: Option<String>,
}

pub type SharedStats = Arc<RwLock<Stats>>;

pub fn spawn_status_server(
    stats: SharedStats,
    status_bind: SocketAddr,
    allowed_origins: &[String],
) {
    let origin_values: Vec<HeaderValue> = allowed_origins
        .iter()
        .filter_map(|origin| match HeaderValue::from_str(origin) {
            Ok(v) => Some(v),
            Err(err) => {
                warn!(origin = %origin, error = %err, "invalid CORS origin in config; skipping");
                None
            }
        })
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origin_values))
        .allow_methods([Method::GET])
        .allow_headers([header::CONTENT_TYPE]);

    let status_router = Router::new()
        .route("/", get(handle_index))
        .route("/status.json", get(handle_status_json))
        .layer(cors)
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
    let (
        our_did,
        endpoint_id,
        ipfs_requests,
        rpc_requests,
        uptime,
        ipfs_enabled,
        entity_names,
        root_cid,
    ) = {
        let s = stats.read().await;
        (
            s.our_did.clone(),
            s.endpoint_id.clone(),
            s.ipfs_requests,
            s.rpc_requests,
            now_unix_secs().saturating_sub(s.started_at),
            s.ipfs_publisher_enabled,
            s.entity_names.clone(),
            s.root_cid.clone(),
        )
    };
    let ipfs_status = if ipfs_enabled { "enabled" } else { "disabled" };
    let entities_html = if entity_names.is_empty() {
        "<em>none</em>".to_string()
    } else {
        entity_names
            .iter()
            .map(|n| format!("<code>#{n}</code>"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let ipns_html = did_to_ipns_path(&our_did).unwrap_or_else(|| "-".to_string());
    let root_cid_html = root_cid.as_deref().unwrap_or("-").to_string();
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
<tr><td>IPNS</td><td>{ipns_html}</td></tr>
<tr><td>Endpoint ID (iroh)</td><td>{endpoint_id}</td></tr>
<tr><td>Uptime (seconds)</td><td>{uptime}</td></tr>
<tr><td>IPFS publisher</td><td>{ipfs_status}</td></tr>
<tr><td>IPFS publish requests</td><td>{ipfs_requests}</td></tr>
<tr><td>RPC requests</td><td>{rpc_requests}</td></tr>
<tr><td>Entities</td><td>{entities_html}</td></tr>
<tr><td>Runtime</td><td>{root_cid_html}</td></tr>
</table>
<p><a href="/status.json">status.json</a></p>
</body></html>"#
    );
    Html(html)
}

async fn handle_status_json(State(stats): State<SharedStats>) -> impl IntoResponse {
    let (
        our_did,
        endpoint_id,
        ipfs_requests,
        rpc_requests,
        started_at,
        uptime,
        ipfs_enabled,
        entity_names,
        root_cid,
    ) = {
        let s = stats.read().await;
        (
            s.our_did.clone(),
            s.endpoint_id.clone(),
            s.ipfs_requests,
            s.rpc_requests,
            s.started_at,
            now_unix_secs().saturating_sub(s.started_at),
            s.ipfs_publisher_enabled,
            s.entity_names.clone(),
            s.root_cid.clone(),
        )
    };
    let runtime: Value = root_cid.map_or(Value::Null, |cid| json!({ "/": cid }));
    let ipns = did_to_ipns_path(&our_did);
    let body = json!({
        "did": our_did,
        "ipns": ipns,
        "endpoint_id": endpoint_id,
        "uptime_secs": uptime,
        "ipfs_publisher": ipfs_enabled,
        "ipfs_requests": ipfs_requests,
        "rpc_requests": rpc_requests,
        "started_at": started_at,
        "entity_names": entity_names,
        "runtime": runtime,
    });
    (
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        body.to_string(),
    )
}

fn did_to_ipns_path(did: &str) -> Option<String> {
    let identity = did.strip_prefix("did:ma:")?;
    Some(format!("/ipns/{identity}"))
}
