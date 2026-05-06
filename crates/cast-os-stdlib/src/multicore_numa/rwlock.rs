//! `rwlock` — shared read/exclusive write lock.

/// Sentinel for `rwlock`.
pub struct Rwlock;

cast::concept! {
    name: "rwlock",
    summary: "shared read/exclusive write lock.",
    anchors: [cast_os_stdlib::multicore_numa::rwlock::Rwlock],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
