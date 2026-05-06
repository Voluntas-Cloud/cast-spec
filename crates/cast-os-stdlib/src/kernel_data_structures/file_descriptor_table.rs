//! `file_descriptor_table` — process table mapping handles to open files.

/// Sentinel for `file_descriptor_table`.
pub struct FileDescriptorTable;

cast::concept! {
    name: "file_descriptor_table",
    summary: "process table mapping handles to open files.",
    anchors: [cast_os_stdlib::kernel_data_structures::file_descriptor_table::FileDescriptorTable],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
