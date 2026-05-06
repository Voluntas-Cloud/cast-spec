//! `rt_safe_driver_model` — drivers designed for deterministic behavior.

/// Sentinel for `rt_safe_driver_model`.
pub struct RtSafeDriverModel;

cast::concept! {
    name: "rt_safe_driver_model",
    summary: "drivers designed for deterministic behavior.",
    anchors: [cast_os_stdlib::realtime::rt_safe_driver_model::RtSafeDriverModel],
    tags: ["cast_os_stdlib", "realtime"],
}
