//! `block_device_layer` — abstraction over block storage.

/// Sentinel for `block_device_layer`.
pub struct BlockDeviceLayer;

cast::concept! {
    name: "block_device_layer",
    summary: "abstraction over block storage.",
    anchors: [cast_os_stdlib::filesystem_storage::block_device_layer::BlockDeviceLayer],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
