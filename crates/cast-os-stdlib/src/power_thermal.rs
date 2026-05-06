//! Power, thermal, and hardware management.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod acpi_power_control;
pub mod battery_management;
pub mod cpu_frequency_scaling;
pub mod cpu_idle_state_management;
pub mod device_suspend_resume;
pub mod energy_model_scheduler_input;
pub mod hibernation_image;
pub mod platform_firmware_interface;
pub mod power_management_subsystem;
pub mod power_profile_policy;
pub mod runtime_power_management;
pub mod system_sleep_state;
pub mod thermal_throttling;
pub mod thermal_zone_model;
pub mod wake_source_tracking;

cast::concept! {
    name: "power_thermal",
    summary: "Umbrella for the power_thermal stdlib category. Power, \
              thermal, and hardware management.",
    anchors: [
        crate::power_thermal::acpi_power_control,
        crate::power_thermal::battery_management,
        crate::power_thermal::cpu_frequency_scaling,
        crate::power_thermal::cpu_idle_state_management,
        crate::power_thermal::device_suspend_resume,
        crate::power_thermal::energy_model_scheduler_input,
        crate::power_thermal::hibernation_image,
        crate::power_thermal::platform_firmware_interface,
        crate::power_thermal::power_management_subsystem,
        crate::power_thermal::power_profile_policy,
        crate::power_thermal::runtime_power_management,
        crate::power_thermal::system_sleep_state,
        crate::power_thermal::thermal_throttling,
        crate::power_thermal::thermal_zone_model,
        crate::power_thermal::wake_source_tracking,
    ],
    tags: ["cast_os_stdlib", "power_thermal"],
}

/// Sentinel for the power_thermal stdlib group.
pub struct PowerThermalGroup;
