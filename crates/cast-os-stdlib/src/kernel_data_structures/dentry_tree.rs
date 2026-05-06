//! `dentry_tree` — directory entry cache/tree.

/// Sentinel for `dentry_tree`.
pub struct DentryTree;

cast::concept! {
    name: "dentry_tree",
    summary: "directory entry cache/tree.",
    anchors: [cast_os_stdlib::kernel_data_structures::dentry_tree::DentryTree],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
