//! `major_page_fault` — fault requiring disk I/O.

/// Sentinel for `major_page_fault`.
pub struct MajorPageFault;

cast::concept! {
    name: "major_page_fault",
    summary: "fault requiring disk I/O.",
    anchors: [cast_os_stdlib::memory_management::major_page_fault::MajorPageFault],
    tags: ["cast_os_stdlib", "memory_management"],
}
