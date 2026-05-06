//! `minor_page_fault` — fault resolved without disk I/O.

/// Sentinel for `minor_page_fault`.
pub struct MinorPageFault;

cast::concept! {
    name: "minor_page_fault",
    summary: "fault resolved without disk I/O.",
    anchors: [cast_os_stdlib::memory_management::minor_page_fault::MinorPageFault],
    tags: ["cast_os_stdlib", "memory_management"],
}
