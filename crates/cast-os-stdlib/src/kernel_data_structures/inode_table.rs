//! `inode_table` — filesystem object metadata cache.

/// Sentinel for `inode_table`.
pub struct InodeTable;

cast::concept! {
    name: "inode_table",
    summary: "filesystem object metadata cache.",
    anchors: [cast_os_stdlib::kernel_data_structures::inode_table::InodeTable],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
