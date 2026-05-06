//! `radix_tree_index` — indexed kernel lookup structure.

/// Sentinel for `radix_tree_index`.
pub struct RadixTreeIndex;

cast::concept! {
    name: "radix_tree_index",
    summary: "indexed kernel lookup structure.",
    anchors: [cast_os_stdlib::kernel_data_structures::radix_tree_index::RadixTreeIndex],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
