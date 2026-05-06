//! `page_fault` — exception when memory mapping is missing or invalid.

/// Sentinel for `page_fault`.
pub struct PageFault;

cast::concept! {
    name: "page_fault",
    summary: "exception when memory mapping is missing or invalid.",
    anchors: [cast_os_stdlib::memory_management::page_fault::PageFault],
    tags: ["cast_os_stdlib", "memory_management"],
}
