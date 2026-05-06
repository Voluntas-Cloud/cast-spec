//! `mount_namespace` — isolate filesystem mount view.

/// Sentinel for `mount_namespace`.
pub struct MountNamespace;

cast::concept! {
    name: "mount_namespace",
    summary: "isolate filesystem mount view.",
    anchors: [cast_os_stdlib::isolation::mount_namespace::MountNamespace],
    tags: ["cast_os_stdlib", "isolation"],
}
