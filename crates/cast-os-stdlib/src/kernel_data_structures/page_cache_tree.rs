//! `page_cache_tree` — cached file page mapping.

/// Sentinel for `page_cache_tree`.
pub struct PageCacheTree;

cast::concept! {
    name: "page_cache_tree",
    summary: "cached file page mapping.",
    anchors: [cast_os_stdlib::kernel_data_structures::page_cache_tree::PageCacheTree],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
