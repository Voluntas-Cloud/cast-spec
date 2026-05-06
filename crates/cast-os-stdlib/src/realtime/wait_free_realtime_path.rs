//! `wait_free_realtime_path` — guarantee bounded progress.

/// Sentinel for `wait_free_realtime_path`.
pub struct WaitFreeRealtimePath;

cast::concept! {
    name: "wait_free_realtime_path",
    summary: "guarantee bounded progress.",
    anchors: [cast_os_stdlib::realtime::wait_free_realtime_path::WaitFreeRealtimePath],
    tags: ["cast_os_stdlib", "realtime"],
}
