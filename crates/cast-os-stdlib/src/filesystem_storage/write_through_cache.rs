//! `write_through_cache` — writes synchronously propagated.

/// Sentinel for `write_through_cache`.
pub struct WriteThroughCache;

cast::concept! {
    name: "write_through_cache",
    summary: "writes synchronously propagated.",
    anchors: [cast_os_stdlib::filesystem_storage::write_through_cache::WriteThroughCache],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
