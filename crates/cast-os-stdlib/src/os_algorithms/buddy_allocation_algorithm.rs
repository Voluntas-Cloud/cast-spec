//! `buddy_allocation_algorithm` — allocate/free physical pages.

/// Sentinel for `buddy_allocation_algorithm`.
pub struct BuddyAllocationAlgorithm;

cast::concept! {
    name: "buddy_allocation_algorithm",
    summary: "allocate/free physical pages.",
    anchors: [cast_os_stdlib::os_algorithms::buddy_allocation_algorithm::BuddyAllocationAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
