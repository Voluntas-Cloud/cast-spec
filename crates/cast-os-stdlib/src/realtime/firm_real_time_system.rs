//! `firm_real_time_system` — late result has no value.

/// Sentinel for `firm_real_time_system`.
pub struct FirmRealTimeSystem;

cast::concept! {
    name: "firm_real_time_system",
    summary: "late result has no value.",
    anchors: [cast_os_stdlib::realtime::firm_real_time_system::FirmRealTimeSystem],
    tags: ["cast_os_stdlib", "realtime"],
}
