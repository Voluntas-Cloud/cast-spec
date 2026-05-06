//! `device_firmware_hang` — hardware/firmware stops responding.

/// Sentinel for `device_firmware_hang`.
pub struct DeviceFirmwareHang;

cast::concept! {
    name: "device_firmware_hang",
    summary: "hardware/firmware stops responding.",
    anchors: [cast_os_stdlib::failure_modes::device_firmware_hang::DeviceFirmwareHang],
    tags: ["cast_os_stdlib", "failure_modes"],
}
