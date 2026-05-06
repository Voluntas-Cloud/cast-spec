//! `device_mapper_layer` — virtual block device mapping layer.

/// Sentinel for `device_mapper_layer`.
pub struct DeviceMapperLayer;

cast::concept! {
    name: "device_mapper_layer",
    summary: "virtual block device mapping layer.",
    anchors: [cast_os_stdlib::filesystem_storage::device_mapper_layer::DeviceMapperLayer],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
