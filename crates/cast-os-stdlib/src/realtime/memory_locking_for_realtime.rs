//! `memory_locking_for_realtime` — prevent page faults in RT code.

/// Sentinel for `memory_locking_for_realtime`.
pub struct MemoryLockingForRealtime;

cast::concept! {
    name: "memory_locking_for_realtime",
    summary: "prevent page faults in RT code.",
    anchors: [cast_os_stdlib::realtime::memory_locking_for_realtime::MemoryLockingForRealtime],
    tags: ["cast_os_stdlib", "realtime"],
}
