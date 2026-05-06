//! `spinlock` — busy-wait lock.

/// Sentinel for `spinlock`.
pub struct Spinlock;

cast::concept! {
    name: "spinlock",
    summary: "busy-wait lock.",
    anchors: [cast_os_stdlib::multicore_numa::spinlock::Spinlock],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
