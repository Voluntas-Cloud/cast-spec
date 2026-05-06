//! `slab_allocator` — cache allocator for kernel objects.

/// Sentinel for `slab_allocator`.
pub struct SlabAllocator;

cast::concept! {
    name: "slab_allocator",
    summary: "cache allocator for kernel objects.",
    anchors: [cast_os_stdlib::memory_management::slab_allocator::SlabAllocator],
    tags: ["cast_os_stdlib", "memory_management"],
}
