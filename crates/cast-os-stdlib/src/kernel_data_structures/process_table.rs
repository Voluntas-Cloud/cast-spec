//! `process_table` — collection of process records.

/// Sentinel for `process_table`.
pub struct ProcessTable;

cast::concept! {
    name: "process_table",
    summary: "collection of process records.",
    anchors: [cast_os_stdlib::kernel_data_structures::process_table::ProcessTable],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
