//! `virtio_device_model` — paravirtualized device interface.

/// Sentinel for `virtio_device_model`.
pub struct VirtioDeviceModel;

cast::concept! {
    name: "virtio_device_model",
    summary: "paravirtualized device interface.",
    anchors: [cast_os_stdlib::virtualization::virtio_device_model::VirtioDeviceModel],
    tags: ["cast_os_stdlib", "virtualization"],
}
