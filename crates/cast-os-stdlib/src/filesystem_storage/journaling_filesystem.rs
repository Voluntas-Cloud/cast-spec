//! `journaling_filesystem` — filesystem records changes before applying them.

/// Sentinel for `journaling_filesystem`.
pub struct JournalingFilesystem;

cast::concept! {
    name: "journaling_filesystem",
    summary: "filesystem records changes before applying them.",
    anchors: [cast_os_stdlib::filesystem_storage::journaling_filesystem::JournalingFilesystem],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
