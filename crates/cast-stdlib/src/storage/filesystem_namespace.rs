//! `filesystem_namespace` — POSIX-shaped hierarchical naming.

/// Sentinel for `filesystem_namespace`.
pub struct FilesystemNamespace;

cast::concept! {
    name: "filesystem_namespace",
    summary: "Hierarchical naming and organization of files. Paths, \
              directories, partial-write, rename, permissions — the \
              POSIX surface that object storage deliberately omits.",
    anchors: [cast_stdlib::storage::filesystem_namespace::FilesystemNamespace],
    tags: ["cast_stdlib", "storage"],
}
