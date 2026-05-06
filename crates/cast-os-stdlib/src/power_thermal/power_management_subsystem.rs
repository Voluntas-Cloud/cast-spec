//! `power_management_subsystem` — OS power control layer.

/// Sentinel for `power_management_subsystem`.
pub struct PowerManagementSubsystem;

cast::concept! {
    name: "power_management_subsystem",
    summary: "OS power control layer.",
    anchors: [cast_os_stdlib::power_thermal::power_management_subsystem::PowerManagementSubsystem],
    tags: ["cast_os_stdlib", "power_thermal"],
}
