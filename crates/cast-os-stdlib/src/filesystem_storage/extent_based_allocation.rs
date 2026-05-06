//! `extent_based_allocation` — allocate ranges of blocks.

/// Sentinel for `extent_based_allocation`.
pub struct ExtentBasedAllocation;

cast::concept! {
    name: "extent_based_allocation",
    summary: "allocate ranges of blocks.",
    anchors: [cast_os_stdlib::filesystem_storage::extent_based_allocation::ExtentBasedAllocation],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
