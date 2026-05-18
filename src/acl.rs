use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use ma_core::check_op;
use tokio::sync::RwLock;
use tracing::info;

pub use ma_core::{normalize_principal, validate_acl_map, AclMap, PERM_W, PERM_X};

/// In-memory cache of named ACLs, keyed by `"<ns>.acl.<name>"`.
pub type AclCache = Arc<RwLock<HashMap<String, AclMap>>>;

/// Create a new empty [`AclCache`].
pub fn new_acl_cache() -> AclCache {
    Arc::new(RwLock::new(HashMap::new()))
}

/// Returns `true` if `key` matches the `<ns>.acl.<name>` pattern:
/// exactly three dot-separated segments where the middle segment is `"acl"`.
#[allow(dead_code)]
pub fn is_acl_key(key: &str) -> bool {
    let parts: Vec<&str> = key.split('.').collect();
    parts.len() == 3 && parts[1] == "acl"
}

/// Parse a `<ns>.acl.<name>` key into `(ns, name)`.
///
/// Returns `None` if the key does not match the pattern.
#[allow(dead_code)]
pub fn parse_acl_key(key: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = key.split('.').collect();
    if parts.len() == 3 && parts[1] == "acl" {
        Some((parts[0], parts[2]))
    } else {
        None
    }
}

/// Fetch an ACL document by CID from IPFS and parse its `acl` field as [`AclMap`].
///
/// The document must contain an `acl` key with a map of principal → permission string.
/// The `kind` field (e.g. `/ma/acl/0.0.1`) is ignored; only `acl` is extracted.
pub async fn load_acl_from_cid(kubo_rpc_url: &str, cid: &str) -> Result<AclMap> {
    #[derive(serde::Deserialize)]
    struct AclDoc {
        acl: AclMap,
    }
    let doc: AclDoc = crate::kubo::dag_get(kubo_rpc_url, cid)
        .await
        .with_context(|| format!("fetching ACL document {cid}"))?;
    Ok(doc.acl)
}

const MA_DEFAULT_SLUG: &str = "ma";
const OPEN_ACL_YAML: &str = include_str!("../default.acl");

/// Transport-level ACL check.
///
/// Call with the appropriate `required` bits for the service:
/// - `/ma/rpc/0.0.1`  — use `PERM_X`
/// - `/ma/ipfs/0.0.1` — use `PERM_W`
pub fn acl_check(acl: &AclMap, from: &str, required: u8) -> Result<()> {
    check_op(acl, from, required).with_context(|| format!("access denied for {from}"))
}

fn default_acl_path() -> Result<PathBuf> {
    ProjectDirs::from("", "ma", "ma")
        .ok_or_else(|| anyhow::anyhow!("cannot determine XDG base directories"))
        .map(|d| d.config_dir().join(format!("{MA_DEFAULT_SLUG}.acl")))
}

fn parse_acl_yaml(yaml: &str) -> Result<AclMap> {
    #[derive(serde::Deserialize)]
    struct AclFile {
        acl: AclMap,
    }
    let f: AclFile =
        serde_yaml::from_str(yaml).map_err(|e| anyhow::anyhow!("invalid ACL YAML: {e}"))?;
    validate_acl_map(&f.acl).map_err(|e| anyhow::anyhow!("invalid ACL key: {e}"))?;
    Ok(f.acl)
}

pub fn load_acl(explicit: Option<&std::path::Path>) -> Result<AclMap> {
    if let Some(p) = explicit {
        let yaml = std::fs::read_to_string(p)
            .with_context(|| format!("failed to read ACL file {}", p.display()))?;
        info!(path = %p.display(), "ACL loaded from file");
        parse_acl_yaml(&yaml).context("invalid ACL YAML")
    } else {
        let default_path = default_acl_path()?;
        if default_path.exists() {
            let yaml = std::fs::read_to_string(&default_path)
                .with_context(|| format!("failed to read ACL file {}", default_path.display()))?;
            info!(path = %default_path.display(), "ACL loaded from default path");
            parse_acl_yaml(&yaml).context("invalid ACL YAML")
        } else {
            info!(path = %default_path.display(), "no ACL file found, using open access");
            parse_acl_yaml(OPEN_ACL_YAML).context("invalid open ACL")
        }
    }
}

/// Check a verb-level allowlist for `caller`.
///
/// - `"*"` in the list → anyone may call
/// - Otherwise → caller's bare identity must appear explicitly
///
/// The caller's DID is normalised (fragment stripped) before comparison.
#[allow(dead_code)]
pub fn verb_acl_allows(allowlist: &[String], caller: &str) -> bool {
    let normalized = normalize_principal(caller);
    allowlist
        .iter()
        .any(|p| p == "*" || normalize_principal(p) == normalized)
}

#[cfg(test)]
mod tests {
    use ma_core::{check_op, AclMap, Permissions, PERM_R, PERM_W, PERM_X};

    use super::verb_acl_allows;

    fn m(entries: &[(&str, &str)]) -> AclMap {
        entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.parse().expect("valid permissions")))
            .collect()
    }

    #[test]
    fn wildcard_exec_allows_execute() {
        let acl = m(&[("*", "x")]);
        assert!(check_op(&acl, "did:ma:alice", PERM_X).is_ok());
    }

    #[test]
    fn wildcard_exec_denies_write() {
        let acl = m(&[("*", "x")]);
        assert!(check_op(&acl, "did:ma:alice", PERM_W).is_err());
    }

    #[test]
    fn explicit_deny_wins_over_wildcard_allow() {
        let acl = m(&[("*", "rwx"), ("did:ma:bandit", "")]);
        assert!(check_op(&acl, "did:ma:bandit", PERM_X).is_err());
    }

    #[test]
    fn exact_match_restricts_below_wildcard() {
        let acl = m(&[("*", "rwx"), ("did:ma:bob", "r")]);
        assert!(check_op(&acl, "did:ma:bob", PERM_R).is_ok());
        assert!(check_op(&acl, "did:ma:bob", PERM_X).is_err());
    }

    #[test]
    fn did_url_caller_is_normalized() {
        let acl = m(&[("did:ma:alice", "rwx")]);
        assert!(check_op(&acl, "did:ma:alice#sign", PERM_X).is_ok());
    }

    #[test]
    fn default_deny_when_no_matching_entries() {
        assert!(check_op(&AclMap::new(), "did:ma:alice", PERM_X).is_err());
    }

    #[test]
    fn permissions_roundtrip() {
        for (input, expected) in &[
            ("rwx", "rwx"),
            ("r", "r"),
            ("w", "w"),
            ("x", "x"),
            ("rw", "rw"),
            ("rx", "rx"),
            ("wx", "wx"),
            ("", "-"),
        ] {
            let p: Permissions = input.parse().expect("parse");
            assert_eq!(p.to_string(), *expected, "roundtrip for {input:?}");
        }
    }

    #[test]
    fn verb_wildcard_allows_anyone() {
        let list = vec!["*".to_string()];
        assert!(verb_acl_allows(&list, "did:ma:alice"));
        assert!(verb_acl_allows(&list, "did:ma:stranger"));
    }

    #[test]
    fn verb_explicit_allows_listed_caller() {
        let list = vec!["did:ma:alice".to_string()];
        assert!(verb_acl_allows(&list, "did:ma:alice"));
        assert!(!verb_acl_allows(&list, "did:ma:bob"));
    }

    #[test]
    fn verb_empty_list_denies_all() {
        let list: Vec<String> = vec![];
        assert!(!verb_acl_allows(&list, "did:ma:alice"));
    }

    #[test]
    fn verb_acl_normalizes_did_url_caller() {
        let list = vec!["did:ma:alice".to_string()];
        assert!(verb_acl_allows(&list, "did:ma:alice#sign"));
    }
}
