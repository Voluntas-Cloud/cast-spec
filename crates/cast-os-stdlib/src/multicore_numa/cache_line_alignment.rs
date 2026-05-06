//! `cache_line_alignment` — layout data to avoid sharing/pathology.

/// Sentinel for `cache_line_alignment`.
pub struct CacheLineAlignment;

cast::concept! {
    name: "cache_line_alignment",
    summary: "layout data to avoid sharing/pathology.",
    anchors: [cast_os_stdlib::multicore_numa::cache_line_alignment::CacheLineAlignment],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
