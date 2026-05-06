//! `clock_page_replacement` — approximate LRU page replacement.

/// Sentinel for `clock_page_replacement`.
pub struct ClockPageReplacement;

cast::concept! {
    name: "clock_page_replacement",
    summary: "approximate LRU page replacement.",
    anchors: [cast_os_stdlib::memory_management::clock_page_replacement::ClockPageReplacement],
    tags: ["cast_os_stdlib", "memory_management"],
}
