//! `page_table` — maps virtual pages to physical frames.

/// Sentinel for `page_table`.
pub struct PageTable;

cast::concept! {
    name: "page_table",
    summary: "maps virtual pages to physical frames.",
    anchors: [cast_os_stdlib::memory_management::page_table::PageTable],
    tags: ["cast_os_stdlib", "memory_management"],
}
