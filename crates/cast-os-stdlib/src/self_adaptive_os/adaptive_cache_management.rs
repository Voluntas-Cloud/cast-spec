//! `adaptive_cache_management` — cache policy changes with workload.

/// Sentinel for `adaptive_cache_management`.
pub struct AdaptiveCacheManagement;

cast::concept! {
    name: "adaptive_cache_management",
    summary: "cache policy changes with workload.",
    anchors: [cast_os_stdlib::self_adaptive_os::adaptive_cache_management::AdaptiveCacheManagement],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
