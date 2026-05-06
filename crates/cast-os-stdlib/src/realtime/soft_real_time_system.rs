//! `soft_real_time_system` — deadline misses degrade quality.

/// Sentinel for `soft_real_time_system`.
pub struct SoftRealTimeSystem;

cast::concept! {
    name: "soft_real_time_system",
    summary: "deadline misses degrade quality.",
    anchors: [cast_os_stdlib::realtime::soft_real_time_system::SoftRealTimeSystem],
    tags: ["cast_os_stdlib", "realtime"],
}
