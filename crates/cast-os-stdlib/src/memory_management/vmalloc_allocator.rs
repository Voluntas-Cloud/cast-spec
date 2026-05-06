//! `vmalloc_allocator` — virtually contiguous kernel allocation.

/// Sentinel for `vmalloc_allocator`.
pub struct VmallocAllocator;

cast::concept! {
    name: "vmalloc_allocator",
    summary: "virtually contiguous kernel allocation.",
    anchors: [cast_os_stdlib::memory_management::vmalloc_allocator::VmallocAllocator],
    tags: ["cast_os_stdlib", "memory_management"],
}
