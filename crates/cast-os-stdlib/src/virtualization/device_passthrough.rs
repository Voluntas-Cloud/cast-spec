//! `device_passthrough` — assign physical device to guest.

/// Sentinel for `device_passthrough`.
pub struct DevicePassthrough;

cast::concept! {
    name: "device_passthrough",
    summary: "assign physical device to guest.",
    anchors: [cast_os_stdlib::virtualization::device_passthrough::DevicePassthrough],
    tags: ["cast_os_stdlib", "virtualization"],
}
