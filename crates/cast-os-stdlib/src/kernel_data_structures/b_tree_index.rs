//! `b_tree_index` — persistent ordered index.

/// Sentinel for `b_tree_index`.
pub struct BTreeIndex;

cast::concept! {
    name: "b_tree_index",
    summary: "persistent ordered index.",
    anchors: [cast_os_stdlib::kernel_data_structures::b_tree_index::BTreeIndex],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
