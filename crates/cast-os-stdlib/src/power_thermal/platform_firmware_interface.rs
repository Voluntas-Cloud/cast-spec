//! `platform_firmware_interface` — OS talks to firmware/hardware platform.

/// Sentinel for `platform_firmware_interface`.
pub struct PlatformFirmwareInterface;

cast::concept! {
    name: "platform_firmware_interface",
    summary: "OS talks to firmware/hardware platform.",
    anchors: [cast_os_stdlib::power_thermal::platform_firmware_interface::PlatformFirmwareInterface],
    tags: ["cast_os_stdlib", "power_thermal"],
}
