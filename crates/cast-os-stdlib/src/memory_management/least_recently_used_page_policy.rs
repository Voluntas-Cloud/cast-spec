//! `least_recently_used_page_policy` — evict old unused pages.

/// Sentinel for `least_recently_used_page_policy`.
pub struct LeastRecentlyUsedPagePolicy;

cast::concept! {
    name: "least_recently_used_page_policy",
    summary: "evict old unused pages.",
    anchors: [cast_os_stdlib::memory_management::least_recently_used_page_policy::LeastRecentlyUsedPagePolicy],
    tags: ["cast_os_stdlib", "memory_management"],
}
