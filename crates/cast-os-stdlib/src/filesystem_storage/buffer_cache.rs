//! `buffer_cache` — cache for block I/O.

/// Sentinel for `buffer_cache`.
pub struct BufferCache;

cast::concept! {
    name: "buffer_cache",
    summary: "cache for block I/O.",
    anchors: [cast_os_stdlib::filesystem_storage::buffer_cache::BufferCache],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
