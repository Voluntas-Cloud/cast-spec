//! `sysfs_device_model_view` — Linux device/kernel object pseudo-filesystem.

/// Sentinel for `sysfs_device_model_view`.
pub struct SysfsDeviceModelView;

cast::concept! {
    name: "sysfs_device_model_view",
    summary: "Linux device/kernel object pseudo-filesystem.",
    anchors: [cast_os_stdlib::configuration::sysfs_device_model_view::SysfsDeviceModelView],
    tags: ["cast_os_stdlib", "configuration"],
}
