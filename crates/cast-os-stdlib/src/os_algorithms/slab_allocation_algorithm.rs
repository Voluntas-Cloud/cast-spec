//! `slab_allocation_algorithm` — allocate common kernel objects.

/// Sentinel for `slab_allocation_algorithm`.
pub struct SlabAllocationAlgorithm;

cast::concept! {
    name: "slab_allocation_algorithm",
    summary: "allocate common kernel objects.",
    anchors: [cast_os_stdlib::os_algorithms::slab_allocation_algorithm::SlabAllocationAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
