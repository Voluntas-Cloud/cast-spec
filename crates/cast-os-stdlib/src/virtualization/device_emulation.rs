//! `device_emulation` — emulate hardware device in software.

/// Sentinel for `device_emulation`.
pub struct DeviceEmulation;

cast::concept! {
    name: "device_emulation",
    summary: "emulate hardware device in software.",
    anchors: [cast_os_stdlib::virtualization::device_emulation::DeviceEmulation],
    tags: ["cast_os_stdlib", "virtualization"],
}
