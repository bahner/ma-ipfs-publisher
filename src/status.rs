use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::{header, HeaderValue, Method};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use tokio::sync::RwLock;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{info, warn};

use crate::acl::SharedAcl;

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
    pub kubo_rpc_url: String,
    /// DIDs that own this runtime; set via --owner / config or POST /claim.
    pub owners: Vec<String>,
    /// Path to config.yaml; used by /claim to persist owners.
    pub config_path: Option<PathBuf>,
}

pub type SharedStats = Arc<RwLock<Stats>>;

/// Combined axum router state for the status server.
#[derive(Clone)]
pub struct StatusState {
    pub stats: SharedStats,
    pub acl: SharedAcl,
}

pub fn spawn_status_server(
    stats: SharedStats,
    acl: SharedAcl,
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
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let status_router = Router::new()
        .route("/", get(handle_index))
        .route("/status.json", get(handle_status_json))
        .route("/bootstrap.yaml", get(handle_bootstrap_yaml))
        .route("/claim", post(handle_claim))
        .layer(cors)
        .with_state(StatusState { stats, acl });

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

async fn handle_index(State(state): State<StatusState>) -> impl IntoResponse {
    let stats = &state.stats;
    let (
        our_did,
        endpoint_id,
        ipfs_requests,
        rpc_requests,
        uptime,
        ipfs_enabled,
        entity_names,
        root_cid,
        owners,
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
            s.owners.clone(),
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
    let owner_html = if owners.is_empty() {
        "<em>unclaimed</em>".to_string()
    } else {
        owners.join("<br>")
    };
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
<tr><td>Owner</td><td>{owner_html}</td></tr>
</table>
<p><a href="/status.json">status.json</a> &bull; <a href="/bootstrap.yaml">bootstrap.yaml</a></p>
</body></html>"#
    );
    Html(html)
}

async fn handle_status_json(State(state): State<StatusState>) -> impl IntoResponse {
    let stats = &state.stats;
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

async fn handle_bootstrap_yaml(State(state): State<StatusState>) -> impl IntoResponse {
    let stats = &state.stats;
    let (root_cid, kubo_rpc_url) = {
        let s = stats.read().await;
        (s.root_cid.clone(), s.kubo_rpc_url.clone())
    };

    let Some(cid) = root_cid else {
        return (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            [(axum::http::header::CONTENT_TYPE, "text/plain")],
            "Runtime not yet initialised (no root CID)".to_string(),
        )
            .into_response();
    };

    match crate::bootstrap::export_bootstrap_yaml(&cid, &kubo_rpc_url).await {
        Ok(yaml) => (
            axum::http::StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "text/yaml; charset=utf-8")],
            yaml,
        )
            .into_response(),
        Err(e) => {
            warn!(error = %e, "failed to export bootstrap.yaml");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                [(axum::http::header::CONTENT_TYPE, "text/plain")],
                format!("export failed: {e}"),
            )
                .into_response()
        }
    }
}

#[derive(serde::Deserialize)]
struct ClaimBody {
    owner: String,
}

async fn handle_claim(
    State(state): State<StatusState>,
    Json(body): Json<ClaimBody>,
) -> impl IntoResponse {
    let (already_claimed, config_path) = {
        let s = state.stats.read().await;
        (!s.owners.is_empty(), s.config_path.clone())
    };

    if already_claimed {
        return (
            axum::http::StatusCode::CONFLICT,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            serde_json::json!({"error": "already claimed"}).to_string(),
        )
            .into_response();
    }

    let new_owners = vec![body.owner.clone()];
    {
        let mut s = state.stats.write().await;
        s.owners = new_owners.clone();
    }
    info!(owner = %body.owner, "{}", crate::i18n::t("runtime-claimed"));

    // Grant owner all capabilities in the live root ACL.
    grant_owners_in_acl(&state.acl, &new_owners).await;

    // Persist to config.yaml so the claim survives a restart.
    if let Some(ref path) = config_path {
        match persist_owners_to_config(path, &new_owners) {
            Ok(()) => info!("{}", crate::i18n::t("runtime-claim-persisted")),
            Err(e) => warn!(error = %e, "failed to persist owners to config.yaml"),
        }
    }

    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        serde_json::json!({"owners": new_owners, "status": "claimed"}).to_string(),
    )
        .into_response()
}

/// Write the `owner` key as a YAML sequence into the config file at `path`.
///
/// The file is read as a raw YAML mapping, the key is inserted or updated,
/// and the result is written back with mode 0600.
fn persist_owners_to_config(path: &std::path::Path, owners: &[String]) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let existing = if path.exists() {
        std::fs::read_to_string(path)?
    } else {
        String::new()
    };

    let mut mapping: serde_yaml::Mapping = if existing.is_empty() {
        serde_yaml::Mapping::new()
    } else {
        let val: serde_yaml::Value = serde_yaml::from_str(&existing)?;
        match val {
            serde_yaml::Value::Mapping(m) => m,
            _ => return Err(anyhow::anyhow!("config file is not a YAML mapping")),
        }
    };

    let seq = serde_yaml::Value::Sequence(
        owners
            .iter()
            .map(|d| serde_yaml::Value::String(d.clone()))
            .collect(),
    );
    mapping.insert(serde_yaml::Value::String("owners".to_string()), seq);

    let yaml_text = serde_yaml::to_string(&serde_yaml::Value::Mapping(mapping))?;
    std::fs::write(path, &yaml_text)?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    Ok(())
}

/// Grant `owner` all capabilities (`"*"`) in the shared root ACL.
///
/// Called both at startup (when owner is already configured) and when
/// `POST /claim` fires, so the owner can immediately use RPC even before
/// any manifest ACL has been published.
/// Grant every DID in `owners` all capabilities (`["*"]`) in the shared root ACL.
///
/// Called at startup (from known owners list) and when `POST /claim` fires.
pub async fn grant_owners_in_acl(acl: &SharedAcl, owners: &[String]) {
    use ma_core::CapabilityEntry;
    let wildcard: std::collections::BTreeSet<String> =
        std::iter::once("*".to_string()).collect();
    let mut map = acl.write().await;
    for owner in owners {
        map.insert(owner.clone(), CapabilityEntry::Allow(wildcard.clone()));
    }
}
