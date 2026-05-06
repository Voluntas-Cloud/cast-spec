//! `lock_granularity` — scope of lock contention.

/// Sentinel for `lock_granularity`.
pub struct LockGranularity;

cast::concept! {
    name: "lock_granularity",
    summary: "scope of lock contention.",
    anchors: [cast_os_stdlib::multicore_numa::lock_granularity::LockGranularity],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
