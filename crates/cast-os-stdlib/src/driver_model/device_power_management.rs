//! `device_power_management` — suspend/resume and power state handling.

/// Sentinel for `device_power_management`.
pub struct DevicePowerManagement;

cast::concept! {
    name: "device_power_management",
    summary: "suspend/resume and power state handling.",
    anchors: [cast_os_stdlib::driver_model::device_power_management::DevicePowerManagement],
    tags: ["cast_os_stdlib", "driver_model"],
}
