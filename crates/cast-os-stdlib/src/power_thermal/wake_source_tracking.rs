//! `wake_source_tracking` — determine what wakes system.

/// Sentinel for `wake_source_tracking`.
pub struct WakeSourceTracking;

cast::concept! {
    name: "wake_source_tracking",
    summary: "determine what wakes system.",
    anchors: [cast_os_stdlib::power_thermal::wake_source_tracking::WakeSourceTracking],
    tags: ["cast_os_stdlib", "power_thermal"],
}
