//! `inode_model` — file metadata object model.

/// Sentinel for `inode_model`.
pub struct InodeModel;

cast::concept! {
    name: "inode_model",
    summary: "file metadata object model.",
    anchors: [cast_os_stdlib::filesystem_storage::inode_model::InodeModel],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
