use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context, Result};
use ma_core::check_cap;
use tokio::sync::RwLock;

pub use ma_core::{
    normalize_principal, AclMap, CapabilityEntry, CAP_CRUD, CAP_IPFS, CAP_RPC, GROUP_PREFIX,
};

/// The group principal that must always be present in the root transport-gate ACL.
///
/// Removing this entry is rejected by the CRUD handler so that the owners
/// group cannot inadvertently lose access to the runtime.
pub const OWNERS_PRINCIPAL: &str = "+owners";

/// In-memory cache of named ACLs.
///
/// Key conventions:
/// | Key | Meaning |
/// |-----|---------|
/// | `"<ns>.acl"` | Namespace gate `AclMap` |
/// | `"<ns>.acls.<name>"` | Namespace verb-ACL library entry |
/// | `"acls.<name>"` | Root verb-ACL library entry |
pub type AclCache = Arc<RwLock<HashMap<String, AclMap>>>;

/// Create a new empty [`AclCache`].
pub fn new_acl_cache() -> AclCache {
    Arc::new(RwLock::new(HashMap::new()))
}

/// Fetch an ACL document by CID from IPFS and deserialise it as [`AclMap`].
///
/// The document is stored as a raw DAG-CBOR map of principal → capabilities.
pub async fn load_acl_from_cid(kubo_rpc_url: &str, cid: &str) -> Result<AclMap> {
    crate::kubo::dag_get::<AclMap>(kubo_rpc_url, cid)
        .await
        .with_context(|| format!("fetching ACL document {cid}"))
}

/// Shared, mutable root transport-gate ACL.
///
/// An empty map means deny-all — nothing is allowed until a manifest or
/// owner explicitly sets an ACL.
pub type SharedAcl = Arc<RwLock<AclMap>>;

/// Create a new [`SharedAcl`] from an initial [`AclMap`].
pub fn new_shared_acl(map: AclMap) -> SharedAcl {
    Arc::new(RwLock::new(map))
}

/// Uniform async ACL check with group expansion.
///
/// Evaluation order:
/// 1. Explicit deny for `caller` or `"*"` → `Err` immediately (deny always wins).
/// 2. `check_cap(acl, caller, cap)` for each cap in `caps` — first `Ok` returns.
/// 3. For each `+group` key with `Deny`: expand members, caller in group? → `Err`.
/// 4. For each `+group` key with `Allow`: expand members, caller in group and has cap? → `Ok`.
/// 5. Nothing matched → `Err`.
///
/// `caps` uses OR semantics — the caller only needs to satisfy one.
///
/// Use `|_| async { Ok(vec![]) }` as `resolve_group` when group resolution
/// is not available (e.g. at the transport gate before the manifest loads).
pub async fn check_full<F, Fut>(
    acl: &AclMap,
    caller: &str,
    caps: &[&str],
    resolve_group: F,
) -> Result<()>
where
    F: Fn(&str) -> Fut,
    Fut: std::future::Future<Output = Result<Vec<String>>>,
{
    let normalized = normalize_principal(caller);

    // Step 1 — explicit deny wins unconditionally.
    if matches!(acl.get(normalized), Some(CapabilityEntry::Deny)) {
        return Err(anyhow::anyhow!("access denied for {caller}"));
    }
    if matches!(acl.get("*"), Some(CapabilityEntry::Deny)) {
        return Err(anyhow::anyhow!(
            "access denied for {caller} (wildcard deny)"
        ));
    }

    // Step 2 — synchronous Allow check (handles principal entries + wildcard).
    for &cap in caps {
        if check_cap(acl, caller, cap).is_ok() {
            return Ok(());
        }
    }

    // Step 3 — group principal expansion (async).
    // 3a: deny groups checked first (deny wins).
    for (key, entry) in acl {
        if !key.starts_with(GROUP_PREFIX) || !matches!(entry, CapabilityEntry::Deny) {
            continue;
        }
        let members = resolve_group(key).await.unwrap_or_default();
        if members.iter().any(|m| normalize_principal(m) == normalized) {
            return Err(anyhow::anyhow!("access denied for {caller} (group {key})"));
        }
    }
    // 3b: allow groups.
    for (key, entry) in acl {
        if !key.starts_with(GROUP_PREFIX) {
            continue;
        }
        if let CapabilityEntry::Allow(cap_set) = entry {
            let members = resolve_group(key).await.unwrap_or_default();
            if members.iter().any(|m| normalize_principal(m) == normalized)
                && caps
                    .iter()
                    .any(|c| cap_set.contains(*c) || cap_set.contains("*"))
            {
                return Ok(());
            }
        }
    }

    Err(anyhow::anyhow!(
        "access denied for {caller}: none of {caps:?} permitted"
    ))
}

/// Resolve a `+<handle>.<path>` group reference to a list of member DIDs.
///
/// The dot-separated `<path>` after the handle may be of arbitrary depth
/// (e.g. `+alice.project4.admins`). Each dot is converted to a `/` to form
/// the IPFS DAG sub-path: `/ipfs/<root_cid>/<handle>/<path…>`.
///
/// Fetches via `ipfs dag resolve` + `ipfs dag get` and deserialises as
/// `Vec<String>`.
pub async fn fetch_group_members(
    kubo_rpc_url: &str,
    group_ref: &str,
    root_cid: &str,
) -> Result<Vec<String>> {
    let without_prefix = group_ref
        .strip_prefix(GROUP_PREFIX)
        .ok_or_else(|| anyhow::anyhow!("invalid group ref: {group_ref}"))?;

    // "handle.name" → "/ipfs/<root_cid>/handle/name"
    let subpath: String = without_prefix.replace('.', "/");
    let ipfs_path = format!("/ipfs/{root_cid}/{subpath}");

    let resolved = crate::kubo::dag_resolve(kubo_rpc_url, &ipfs_path)
        .await
        .with_context(|| format!("resolving group path {ipfs_path}"))?;

    crate::kubo::dag_get::<Vec<String>>(kubo_rpc_url, &resolved)
        .await
        .with_context(|| format!("fetching group members at {resolved}"))
}

#[cfg(test)]
mod tests {
    use ma_core::{check_cap, AclMap, CapabilityEntry, CAP_IPFS, CAP_RPC};

    fn allow(caps: &[&str]) -> CapabilityEntry {
        CapabilityEntry::from_caps(caps.iter().copied())
    }

    fn m(entries: &[(&str, CapabilityEntry)]) -> AclMap {
        entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn wildcard_rpc_allows_rpc() {
        let acl = m(&[("*", allow(&["rpc"]))]);
        assert!(check_cap(&acl, "did:ma:alice", CAP_RPC).is_ok());
    }

    #[test]
    fn wildcard_rpc_denies_ipfs() {
        let acl = m(&[("*", allow(&["rpc"]))]);
        assert!(check_cap(&acl, "did:ma:alice", CAP_IPFS).is_err());
    }

    #[test]
    fn explicit_deny_wins_over_wildcard_allow() {
        let acl = m(&[
            ("*", allow(&["rpc", "ipfs"])),
            ("did:ma:bandit", CapabilityEntry::Deny),
        ]);
        assert!(check_cap(&acl, "did:ma:bandit", CAP_RPC).is_err());
    }

    #[test]
    fn exact_match_restricts_below_wildcard() {
        let acl = m(&[
            ("*", allow(&["rpc", "ipfs"])),
            ("did:ma:bob", allow(&["rpc"])),
        ]);
        assert!(check_cap(&acl, "did:ma:bob", CAP_RPC).is_ok());
        assert!(check_cap(&acl, "did:ma:bob", CAP_IPFS).is_err());
    }

    #[test]
    fn did_url_caller_is_normalized() {
        let acl = m(&[("did:ma:alice", allow(&["rpc", "ipfs"]))]);
        assert!(check_cap(&acl, "did:ma:alice#sign", CAP_RPC).is_ok());
    }

    #[test]
    fn default_deny_when_no_matching_entries() {
        assert!(check_cap(&AclMap::new(), "did:ma:alice", CAP_RPC).is_err());
    }

    #[tokio::test]
    async fn check_full_or_semantics() {
        // Caller has "write"; asking for ["read", "write"] — first matching cap wins.
        use super::check_full;
        let acl = m(&[("did:ma:alice", allow(&["write"]))]);
        let result = check_full(&acl, "did:ma:alice", &["read", "write"], |_| async {
            Ok(vec![])
        })
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn check_full_direct_deny_beats_group_allow() {
        // Alice is directly denied AND a member of an allow group — direct deny wins.
        use super::check_full;
        let acl = m(&[
            ("did:ma:alice", CapabilityEntry::Deny),
            ("+carlotta.friends", allow(&["fortune"])),
        ]);
        let result = check_full(&acl, "did:ma:alice", &["fortune"], |g| {
            let g = g.to_string();
            async move {
                if g == "+carlotta.friends" {
                    Ok(vec!["did:ma:alice".to_string()])
                } else {
                    Ok(vec![])
                }
            }
        })
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn check_full_groups_accumulate_caps() {
        // Alice is in two groups with different caps; she should have both.
        use super::check_full;
        let acl = m(&[
            ("+project.readers", allow(&["read"])),
            ("+project.writers", allow(&["write"])),
        ]);
        let resolve = |_g: &str| async move { Ok(vec!["did:ma:alice".to_string()]) };
        // Check read via readers group.
        let r1 = check_full(&acl, "did:ma:alice", &["read"], resolve).await;
        assert!(r1.is_ok());
        // Check write via writers group.
        let r2 = check_full(&acl, "did:ma:alice", &["write"], resolve).await;
        assert!(r2.is_ok());
        // Bob is not in any group.
        let r3 = check_full(&acl, "did:ma:bob", &["read"], |_| async { Ok(vec![]) }).await;
        assert!(r3.is_err());
    }

    #[tokio::test]
    async fn check_full_deep_group_path() {
        // Groups can have arbitrary depth: +alice.project4.admins
        use super::check_full;
        let acl = m(&[("+alice.project4.admins", allow(&["admin"]))]);
        let result = check_full(&acl, "did:ma:alice", &["admin"], |g| {
            let g = g.to_string();
            async move {
                if g == "+alice.project4.admins" {
                    Ok(vec!["did:ma:alice".to_string()])
                } else {
                    Ok(vec![])
                }
            }
        })
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn check_full_allow_via_allow_entry() {
        use super::check_full;
        let acl = m(&[("did:ma:alice", allow(&["read"]))]);
        let result = check_full(&acl, "did:ma:alice", &["read"], |_| async { Ok(vec![]) }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn check_full_deny_wins_over_wildcard_allow() {
        use super::check_full;
        let acl = m(&[
            ("*", allow(&["rpc", "ipfs"])),
            ("did:ma:bandit", CapabilityEntry::Deny),
        ]);
        let result = check_full(&acl, "did:ma:bandit", &["rpc"], |_| async { Ok(vec![]) }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn check_full_group_allow() {
        use super::check_full;
        let acl = m(&[("+carlotta.friends", allow(&["fortune"]))]);
        let result = check_full(&acl, "did:ma:alice", &["fortune"], |g| {
            let g = g.to_string();
            async move {
                if g == "+carlotta.friends" {
                    Ok(vec!["did:ma:alice".to_string()])
                } else {
                    Ok(vec![])
                }
            }
        })
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn check_full_group_deny() {
        use super::check_full;
        let acl = m(&[("+alice.enemies", CapabilityEntry::Deny)]);
        let result = check_full(&acl, "did:ma:alice", &["fortune"], |g| {
            let g = g.to_string();
            async move {
                if g == "+alice.enemies" {
                    Ok(vec!["did:ma:alice".to_string()])
                } else {
                    Ok(vec![])
                }
            }
        })
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn check_full_group_deny_wins_over_group_allow() {
        use super::check_full;
        let acl = m(&[
            ("+alice.enemies", CapabilityEntry::Deny),
            ("+alice.friends", allow(&["fortune"])),
        ]);
        // alice is in both groups — deny wins.
        let result = check_full(&acl, "did:ma:alice", &["fortune"], |g| {
            let _g = g.to_string();
            async move { Ok(vec!["did:ma:alice".to_string()]) }
        })
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn check_full_group_non_member_denied() {
        use super::check_full;
        let acl = m(&[("+carlotta.friends", allow(&["fortune"]))]);
        let result = check_full(&acl, "did:ma:stranger", &["fortune"], |_| async {
            Ok(vec![])
        })
        .await;
        assert!(result.is_err());
    }
}
