//! `dentry_cache` — cached directory lookup entries.

/// Sentinel for `dentry_cache`.
pub struct DentryCache;

cast::concept! {
    name: "dentry_cache",
    summary: "cached directory lookup entries.",
    anchors: [cast_os_stdlib::filesystem_storage::dentry_cache::DentryCache],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
