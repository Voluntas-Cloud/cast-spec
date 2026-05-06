//! `page_cache` — memory cache for file contents.

/// Sentinel for `page_cache`.
pub struct PageCache;

cast::concept! {
    name: "page_cache",
    summary: "memory cache for file contents.",
    anchors: [cast_os_stdlib::filesystem_storage::page_cache::PageCache],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
