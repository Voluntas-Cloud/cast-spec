//! `buddy_allocator_tree` — physical page allocation structure.

/// Sentinel for `buddy_allocator_tree`.
pub struct BuddyAllocatorTree;

cast::concept! {
    name: "buddy_allocator_tree",
    summary: "physical page allocation structure.",
    anchors: [cast_os_stdlib::kernel_data_structures::buddy_allocator_tree::BuddyAllocatorTree],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
