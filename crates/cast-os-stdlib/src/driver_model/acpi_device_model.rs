//! `acpi_device_model` — firmware-provided hardware description/control.

/// Sentinel for `acpi_device_model`.
pub struct AcpiDeviceModel;

cast::concept! {
    name: "acpi_device_model",
    summary: "firmware-provided hardware description/control.",
    anchors: [cast_os_stdlib::driver_model::acpi_device_model::AcpiDeviceModel],
    tags: ["cast_os_stdlib", "driver_model"],
}
