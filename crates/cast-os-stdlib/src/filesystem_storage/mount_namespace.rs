//! `mount_namespace` — isolated mount view.

/// Sentinel for `mount_namespace`.
pub struct MountNamespace;

cast::concept! {
    name: "mount_namespace",
    summary: "isolated mount view.",
    anchors: [cast_os_stdlib::filesystem_storage::mount_namespace::MountNamespace],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
