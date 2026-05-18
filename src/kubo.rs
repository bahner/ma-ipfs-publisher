//! Minimal Kubo HTTP API wrappers for DAG operations not re-exported by
//! `ma_core`.  Only `dag_put` and `dag_get` are needed here; other Kubo
//! operations (`ipfs_add`, `cat_bytes`) are used directly from `ma_core`.

use anyhow::{anyhow, Result};
use reqwest::multipart;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct DagPutCid {
    #[serde(rename = "/")]
    slash: String,
}

#[derive(Deserialize)]
struct DagPutResponse {
    #[serde(default, rename = "Cid")]
    cid_upper: Option<DagPutCid>,
    #[serde(default)]
    cid: Option<DagPutCid>,
}

#[derive(Deserialize)]
struct DagResolveResponse {
    #[serde(rename = "Cid")]
    cid: DagPutCid,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Publish a serialisable value as a `dag-cbor` IPLD node via Kubo.
/// Input is serialised as `dag-json`; Kubo converts and stores as `dag-cbor`.
/// Returns the resulting CID string.
pub async fn dag_put<T: Serialize + Sync>(kubo_url: &str, value: &T) -> Result<String> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/dag/put");
    let payload = serde_json::to_vec(value)?;

    let part = multipart::Part::bytes(payload)
        .file_name("node.json")
        .mime_str("application/json")?;
    let form = multipart::Form::new().part("file", part);

    let body = reqwest::Client::new()
        .post(url)
        .query(&[
            ("store-codec", "dag-cbor"),
            ("input-codec", "dag-json"),
            ("pin", "false"),
        ])
        .multipart(form)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: DagPutResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing dag/put response: {e} body={body}"))?;
    parsed
        .cid_upper
        .or(parsed.cid)
        .map(|c| c.slash)
        .ok_or_else(|| anyhow!("missing CID in dag/put response: {body}"))
}

/// Recursively pin a CID. Used for first-time bootstrap when there is no
/// prior root to update from.
pub async fn pin_add(kubo_url: &str, cid: &str) -> Result<()> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/pin/add");
    reqwest::Client::new()
        .post(&url)
        .query(&[("arg", cid), ("recursive", "true")])
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

/// Atomically move the recursive pin from `old_cid` to `new_cid` via
/// Kubo's `pin/update` endpoint (`unpin=true`).  If `old_cid` was not
/// pinned, falls back to `pin_add(new_cid)` so first-time callers work too.
pub async fn pin_update(kubo_url: &str, old_cid: &str, new_cid: &str) -> Result<()> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/pin/update");
    let resp = reqwest::Client::new()
        .post(&url)
        .query(&[("arg", old_cid), ("arg", new_cid), ("unpin", "true")])
        .send()
        .await?;
    if resp.status().is_success() {
        return Ok(());
    }
    let body = resp.text().await.unwrap_or_default();
    // If the old CID was never pinned, fall back to a plain recursive pin.
    if body.contains("not pinned") {
        return pin_add(kubo_url, new_cid).await;
    }
    Err(anyhow!("pin/update {old_cid} → {new_cid} failed: {body}"))
}

/// Publish pre-encoded DAG-CBOR bytes as an IPLD node via Kubo.
/// Use this when the caller has already serialised the node (e.g. received
/// it over the wire). Returns the resulting CID string.
pub async fn dag_put_raw(kubo_url: &str, dag_cbor: &[u8]) -> Result<String> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/dag/put");

    let part = multipart::Part::bytes(dag_cbor.to_vec())
        .file_name("node.cbor")
        .mime_str("application/octet-stream")?;
    let form = multipart::Form::new().part("file", part);

    let body = reqwest::Client::new()
        .post(url)
        .query(&[
            ("store-codec", "dag-cbor"),
            ("input-codec", "dag-cbor"),
            ("pin", "false"),
        ])
        .multipart(form)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: DagPutResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing dag/put response: {e} body={body}"))?;
    parsed
        .cid_upper
        .or(parsed.cid)
        .map(|c| c.slash)
        .ok_or_else(|| anyhow!("missing CID in dag/put response: {body}"))
}

/// Fetch an IPLD node from Kubo and deserialise it from `dag-json`.
pub async fn dag_get<T: DeserializeOwned>(kubo_url: &str, cid: &str) -> Result<T> {
    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/dag/get");

    let body = reqwest::Client::new()
        .post(&url)
        .query(&[("arg", cid), ("output-codec", "dag-json")])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed to deserialise dag/get response for {cid}: {e} body={body}"))
}

/// Resolve an IPFS/IPNS path to a bare CID string.
///
/// Accepts bare CIDs (`bafy…`), `/ipfs/<cid>`, and `/ipns/<key>` paths.
/// `/ipns/` paths are resolved through Kubo's name resolution.
pub async fn dag_resolve(kubo_url: &str, path: &str) -> Result<String> {
    // Bare CID — nothing to resolve.
    if !path.starts_with('/') {
        return Ok(path.to_string());
    }

    let base = kubo_url.trim_end_matches('/');
    let url = format!("{base}/api/v0/dag/resolve");

    let body = reqwest::Client::new()
        .post(&url)
        .query(&[("arg", path)])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let parsed: DagResolveResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("failed parsing dag/resolve response for {path}: {e} body={body}"))?;
    Ok(parsed.cid.slash)
}
