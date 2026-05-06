//! `bitmap_allocator` — allocation state as bits.

/// Sentinel for `bitmap_allocator`.
pub struct BitmapAllocator;

cast::concept! {
    name: "bitmap_allocator",
    summary: "allocation state as bits.",
    anchors: [cast_os_stdlib::kernel_data_structures::bitmap_allocator::BitmapAllocator],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
