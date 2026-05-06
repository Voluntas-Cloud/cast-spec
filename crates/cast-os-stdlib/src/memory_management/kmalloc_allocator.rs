//! `kmalloc_allocator` — kernel heap allocation model.

/// Sentinel for `kmalloc_allocator`.
pub struct KmallocAllocator;

cast::concept! {
    name: "kmalloc_allocator",
    summary: "kernel heap allocation model.",
    anchors: [cast_os_stdlib::memory_management::kmalloc_allocator::KmallocAllocator],
    tags: ["cast_os_stdlib", "memory_management"],
}
