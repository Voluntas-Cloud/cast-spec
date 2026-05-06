//! `page_frame` — physical memory unit.

/// Sentinel for `page_frame`.
pub struct PageFrame;

cast::concept! {
    name: "page_frame",
    summary: "physical memory unit.",
    anchors: [cast_os_stdlib::memory_management::page_frame::PageFrame],
    tags: ["cast_os_stdlib", "memory_management"],
}
