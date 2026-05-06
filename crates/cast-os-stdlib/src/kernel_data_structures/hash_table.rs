//! `hash_table` — fast exact lookup.

/// Sentinel for `hash_table`.
pub struct HashTable;

cast::concept! {
    name: "hash_table",
    summary: "fast exact lookup.",
    anchors: [cast_os_stdlib::kernel_data_structures::hash_table::HashTable],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
