//! `buddy_allocator` — physical page allocation algorithm.

/// Sentinel for `buddy_allocator`.
pub struct BuddyAllocator;

cast::concept! {
    name: "buddy_allocator",
    summary: "physical page allocation algorithm.",
    anchors: [cast_os_stdlib::memory_management::buddy_allocator::BuddyAllocator],
    tags: ["cast_os_stdlib", "memory_management"],
}
