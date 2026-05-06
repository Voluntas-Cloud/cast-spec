//! `virtual_filesystem_layer` — common filesystem abstraction.

/// Sentinel for `virtual_filesystem_layer`.
pub struct VirtualFilesystemLayer;

cast::concept! {
    name: "virtual_filesystem_layer",
    summary: "common filesystem abstraction.",
    anchors: [cast_os_stdlib::filesystem_storage::virtual_filesystem_layer::VirtualFilesystemLayer],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
