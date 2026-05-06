//! `hard_real_time_system` — missing deadline is system failure.

/// Sentinel for `hard_real_time_system`.
pub struct HardRealTimeSystem;

cast::concept! {
    name: "hard_real_time_system",
    summary: "missing deadline is system failure.",
    anchors: [cast_os_stdlib::realtime::hard_real_time_system::HardRealTimeSystem],
    tags: ["cast_os_stdlib", "realtime"],
}
