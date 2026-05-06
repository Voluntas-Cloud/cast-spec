//! `writeback_cache` — writes buffered before storage.

/// Sentinel for `writeback_cache`.
pub struct WritebackCache;

cast::concept! {
    name: "writeback_cache",
    summary: "writes buffered before storage.",
    anchors: [cast_os_stdlib::filesystem_storage::writeback_cache::WritebackCache],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
