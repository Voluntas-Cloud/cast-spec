//! `battery_management` — battery state and charging policy.

/// Sentinel for `battery_management`.
pub struct BatteryManagement;

cast::concept! {
    name: "battery_management",
    summary: "battery state and charging policy.",
    anchors: [cast_os_stdlib::power_thermal::battery_management::BatteryManagement],
    tags: ["cast_os_stdlib", "power_thermal"],
}
