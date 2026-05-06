//! `thread_table` — collection of thread records.

/// Sentinel for `thread_table`.
pub struct ThreadTable;

cast::concept! {
    name: "thread_table",
    summary: "collection of thread records.",
    anchors: [cast_os_stdlib::kernel_data_structures::thread_table::ThreadTable],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
