//! Shared helpers for ma-ipfs-publisher.

/// Extracts the canonical imported IPNS id from a ma-core mismatch error text.
///
/// The expected input shape is:
/// `imported key IPNS id '<k-id>' does not match document DID IPNS '<did-id>'`.
///
/// # Examples
///
/// ```
/// use ma_runtime::extract_imported_ipns_id_from_error_text;
///
/// let text = "imported key IPNS id 'k51qzi5uqu5di8zyh33j0mvlj19vwt1wm1z0bztofwebkihfrdobsrzwmxpr77' does not match document DID IPNS '12D3KooWFQ64SYT3CmZTTYsqZyH9qM7bacBpYW7dXKbyBTF9JMMt'";
///
/// assert_eq!(
///     extract_imported_ipns_id_from_error_text(text).as_deref(),
///     Some("k51qzi5uqu5di8zyh33j0mvlj19vwt1wm1z0bztofwebkihfrdobsrzwmxpr77")
/// );
/// ```
#[must_use]
pub fn extract_imported_ipns_id_from_error_text(text: &str) -> Option<String> {
    let prefix = "imported key IPNS id '";
    let infix = "' does not match document DID IPNS '";
    let start = text.find(prefix)? + prefix.len();
    let rest = &text[start..];
    let mid = rest.find(infix)?;
    let imported = rest[..mid].trim();
    if imported.is_empty() {
        None
    } else {
        Some(imported.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::extract_imported_ipns_id_from_error_text;

    #[test]
    fn extracts_canonical_ipns_id_from_mismatch_error() {
        let text = "imported key IPNS id 'k51qzi5uqu5di8zyh33j0mvlj19vwt1wm1z0bztofwebkihfrdobsrzwmxpr77' does not match document DID IPNS '12D3KooWFQ64SYT3CmZTTYsqZyH9qM7bacBpYW7dXKbyBTF9JMMt'";
        assert_eq!(
            extract_imported_ipns_id_from_error_text(text).as_deref(),
            Some("k51qzi5uqu5di8zyh33j0mvlj19vwt1wm1z0bztofwebkihfrdobsrzwmxpr77")
        );
    }

    #[test]
    fn returns_none_for_unrelated_error_text() {
        let text = "kubo publish failed with timeout";
        assert_eq!(extract_imported_ipns_id_from_error_text(text), None);
    }

    #[test]
    fn returns_none_when_imported_id_is_empty() {
        let text =
            "imported key IPNS id '' does not match document DID IPNS '12D3KooWFQ64SYT3CmZTTYsqZyH9qM7bacBpYW7dXKbyBTF9JMMt'";
        assert_eq!(extract_imported_ipns_id_from_error_text(text), None);
    }

    #[test]
    fn handles_wrapped_context_text() {
        let text = "failed to publish own DID document: imported key IPNS id 'k51abc' does not match document DID IPNS '12D3xyz'";
        assert_eq!(
            extract_imported_ipns_id_from_error_text(text).as_deref(),
            Some("k51abc")
        );
    }
}
