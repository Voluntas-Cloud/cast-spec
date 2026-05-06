//! `mutex_lock` — sleeping mutual exclusion.

/// Sentinel for `mutex_lock`.
pub struct MutexLock;

cast::concept! {
    name: "mutex_lock",
    summary: "sleeping mutual exclusion.",
    anchors: [cast_os_stdlib::multicore_numa::mutex_lock::MutexLock],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
