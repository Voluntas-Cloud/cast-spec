//! `page_replacement_algorithm` — choose memory page to evict.

/// Sentinel for `page_replacement_algorithm`.
pub struct PageReplacementAlgorithm;

cast::concept! {
    name: "page_replacement_algorithm",
    summary: "choose memory page to evict.",
    anchors: [cast_os_stdlib::os_algorithms::page_replacement_algorithm::PageReplacementAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
