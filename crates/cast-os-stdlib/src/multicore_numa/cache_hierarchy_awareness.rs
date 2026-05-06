//! `cache_hierarchy_awareness` — account for CPU cache topology.

/// Sentinel for `cache_hierarchy_awareness`.
pub struct CacheHierarchyAwareness;

cast::concept! {
    name: "cache_hierarchy_awareness",
    summary: "account for CPU cache topology.",
    anchors: [cast_os_stdlib::multicore_numa::cache_hierarchy_awareness::CacheHierarchyAwareness],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
