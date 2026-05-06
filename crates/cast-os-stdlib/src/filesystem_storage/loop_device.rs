//! `loop_device` — file exposed as block device.

/// Sentinel for `loop_device`.
pub struct LoopDevice;

cast::concept! {
    name: "loop_device",
    summary: "file exposed as block device.",
    anchors: [cast_os_stdlib::filesystem_storage::loop_device::LoopDevice],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
