//! `copy_on_write_filesystem` — updates write new blocks rather than overwriting.

/// Sentinel for `copy_on_write_filesystem`.
pub struct CopyOnWriteFilesystem;

cast::concept! {
    name: "copy_on_write_filesystem",
    summary: "updates write new blocks rather than overwriting.",
    anchors: [cast_os_stdlib::filesystem_storage::copy_on_write_filesystem::CopyOnWriteFilesystem],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
