//! `object_backed_filesystem` — filesystem interface over object storage.

/// Sentinel for `object_backed_filesystem`.
pub struct ObjectBackedFilesystem;

cast::concept! {
    name: "object_backed_filesystem",
    summary: "filesystem interface over object storage.",
    anchors: [cast_os_stdlib::filesystem_storage::object_backed_filesystem::ObjectBackedFilesystem],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
