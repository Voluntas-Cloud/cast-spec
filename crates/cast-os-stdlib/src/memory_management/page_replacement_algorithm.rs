//! `page_replacement_algorithm` — choose which page to evict.

/// Sentinel for `page_replacement_algorithm`.
pub struct PageReplacementAlgorithm;

cast::concept! {
    name: "page_replacement_algorithm",
    summary: "choose which page to evict.",
    anchors: [cast_os_stdlib::memory_management::page_replacement_algorithm::PageReplacementAlgorithm],
    tags: ["cast_os_stdlib", "memory_management"],
}
