//! `major_minor_device_number` — Unix device identity scheme.

/// Sentinel for `major_minor_device_number`.
pub struct MajorMinorDeviceNumber;

cast::concept! {
    name: "major_minor_device_number",
    summary: "Unix device identity scheme.",
    anchors: [cast_os_stdlib::driver_model::major_minor_device_number::MajorMinorDeviceNumber],
    tags: ["cast_os_stdlib", "driver_model"],
}
