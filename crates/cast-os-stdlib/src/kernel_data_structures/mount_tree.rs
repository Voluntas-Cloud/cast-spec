//! `mount_tree` — filesystem mount hierarchy.

/// Sentinel for `mount_tree`.
pub struct MountTree;

cast::concept! {
    name: "mount_tree",
    summary: "filesystem mount hierarchy.",
    anchors: [cast_os_stdlib::kernel_data_structures::mount_tree::MountTree],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
