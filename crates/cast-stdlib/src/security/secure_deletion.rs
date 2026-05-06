//! `secure_deletion` — remove data according to realistic guarantees.

/// Sentinel for `secure_deletion`.
pub struct SecureDeletion;

cast::concept! {
    name: "secure_deletion",
    summary: "Remove data according to realistic guarantees. \"Deleted\" \
              must be qualified by replicas, backups, indexes, and \
              caches; the spec says what will and won't survive, and \
              the system honours that — not a marketing promise.",
    anchors: [cast_stdlib::security::secure_deletion::SecureDeletion],
    tags: ["cast_stdlib", "security"],
}
