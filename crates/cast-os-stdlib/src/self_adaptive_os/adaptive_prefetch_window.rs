//! `adaptive_prefetch_window` — tune prefetch aggressiveness.

/// Sentinel for `adaptive_prefetch_window`.
pub struct AdaptivePrefetchWindow;

cast::concept! {
    name: "adaptive_prefetch_window",
    summary: "tune prefetch aggressiveness.",
    anchors: [cast_os_stdlib::self_adaptive_os::adaptive_prefetch_window::AdaptivePrefetchWindow],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
