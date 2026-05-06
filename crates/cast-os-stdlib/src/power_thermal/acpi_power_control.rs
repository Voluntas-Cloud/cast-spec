//! `acpi_power_control` — firmware-mediated power management.

/// Sentinel for `acpi_power_control`.
pub struct AcpiPowerControl;

cast::concept! {
    name: "acpi_power_control",
    summary: "firmware-mediated power management.",
    anchors: [cast_os_stdlib::power_thermal::acpi_power_control::AcpiPowerControl],
    tags: ["cast_os_stdlib", "power_thermal"],
}
