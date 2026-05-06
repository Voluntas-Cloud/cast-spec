//! `credential_cache` — cached authentication material.

/// Sentinel for `credential_cache`.
pub struct CredentialCache;

cast::concept! {
    name: "credential_cache",
    summary: "cached authentication material.",
    anchors: [cast_os_stdlib::security::credential_cache::CredentialCache],
    tags: ["cast_os_stdlib", "security"],
}
