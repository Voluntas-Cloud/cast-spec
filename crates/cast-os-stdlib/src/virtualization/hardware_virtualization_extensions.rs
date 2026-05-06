//! `hardware_virtualization_extensions` — CPU support for virtualization.

/// Sentinel for `hardware_virtualization_extensions`.
pub struct HardwareVirtualizationExtensions;

cast::concept! {
    name: "hardware_virtualization_extensions",
    summary: "CPU support for virtualization.",
    anchors: [cast_os_stdlib::virtualization::hardware_virtualization_extensions::HardwareVirtualizationExtensions],
    tags: ["cast_os_stdlib", "virtualization"],
}
