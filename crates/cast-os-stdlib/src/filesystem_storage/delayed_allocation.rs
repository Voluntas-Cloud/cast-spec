//! `delayed_allocation` — defer block allocation until flush.

/// Sentinel for `delayed_allocation`.
pub struct DelayedAllocation;

cast::concept! {
    name: "delayed_allocation",
    summary: "defer block allocation until flush.",
    anchors: [cast_os_stdlib::filesystem_storage::delayed_allocation::DelayedAllocation],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
