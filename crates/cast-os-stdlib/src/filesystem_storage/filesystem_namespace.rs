//! `filesystem_namespace` — global or per-process file hierarchy.

/// Sentinel for `filesystem_namespace`.
pub struct FilesystemNamespace;

cast::concept! {
    name: "filesystem_namespace",
    summary: "global or per-process file hierarchy.",
    anchors: [cast_os_stdlib::filesystem_storage::filesystem_namespace::FilesystemNamespace],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
