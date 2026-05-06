//! `runtime_power_management` — power down idle devices.

/// Sentinel for `runtime_power_management`.
pub struct RuntimePowerManagement;

cast::concept! {
    name: "runtime_power_management",
    summary: "power down idle devices.",
    anchors: [cast_os_stdlib::power_thermal::runtime_power_management::RuntimePowerManagement],
    tags: ["cast_os_stdlib", "power_thermal"],
}
