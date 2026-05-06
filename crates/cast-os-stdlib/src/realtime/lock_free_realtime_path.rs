//! `lock_free_realtime_path` — avoid blocking in timing-critical path.

/// Sentinel for `lock_free_realtime_path`.
pub struct LockFreeRealtimePath;

cast::concept! {
    name: "lock_free_realtime_path",
    summary: "avoid blocking in timing-critical path.",
    anchors: [cast_os_stdlib::realtime::lock_free_realtime_path::LockFreeRealtimePath],
    tags: ["cast_os_stdlib", "realtime"],
}
