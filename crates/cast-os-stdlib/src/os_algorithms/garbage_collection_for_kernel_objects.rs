//! `garbage_collection_for_kernel_objects` — safely reclaim objects.

/// Sentinel for `garbage_collection_for_kernel_objects`.
pub struct GarbageCollectionForKernelObjects;

cast::concept! {
    name: "garbage_collection_for_kernel_objects",
    summary: "safely reclaim objects.",
    anchors: [cast_os_stdlib::os_algorithms::garbage_collection_for_kernel_objects::GarbageCollectionForKernelObjects],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
