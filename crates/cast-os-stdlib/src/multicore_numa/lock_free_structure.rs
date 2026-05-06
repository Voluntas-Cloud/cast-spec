//! `lock_free_structure` — progress without locks.

/// Sentinel for `lock_free_structure`.
pub struct LockFreeStructure;

cast::concept! {
    name: "lock_free_structure",
    summary: "progress without locks.",
    anchors: [cast_os_stdlib::multicore_numa::lock_free_structure::LockFreeStructure],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
