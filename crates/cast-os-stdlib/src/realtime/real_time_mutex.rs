//! `real_time_mutex` — mutex supporting priority inheritance.

/// Sentinel for `real_time_mutex`.
pub struct RealTimeMutex;

cast::concept! {
    name: "real_time_mutex",
    summary: "mutex supporting priority inheritance.",
    anchors: [cast_os_stdlib::realtime::real_time_mutex::RealTimeMutex],
    tags: ["cast_os_stdlib", "realtime"],
}
