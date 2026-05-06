//! `udev_device_event_model` — Linux user-space device event handling.

/// Sentinel for `udev_device_event_model`.
pub struct UdevDeviceEventModel;

cast::concept! {
    name: "udev_device_event_model",
    summary: "Linux user-space device event handling.",
    anchors: [cast_os_stdlib::driver_model::udev_device_event_model::UdevDeviceEventModel],
    tags: ["cast_os_stdlib", "driver_model"],
}
