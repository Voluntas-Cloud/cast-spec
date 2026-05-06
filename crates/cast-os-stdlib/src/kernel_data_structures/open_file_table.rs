//! `open_file_table` — kernel records for open file instances.

/// Sentinel for `open_file_table`.
pub struct OpenFileTable;

cast::concept! {
    name: "open_file_table",
    summary: "kernel records for open file instances.",
    anchors: [cast_os_stdlib::kernel_data_structures::open_file_table::OpenFileTable],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
