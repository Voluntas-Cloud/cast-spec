//! `slab_cache` — object cache allocator structure.

/// Sentinel for `slab_cache`.
pub struct SlabCache;

cast::concept! {
    name: "slab_cache",
    summary: "object cache allocator structure.",
    anchors: [cast_os_stdlib::kernel_data_structures::slab_cache::SlabCache],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
