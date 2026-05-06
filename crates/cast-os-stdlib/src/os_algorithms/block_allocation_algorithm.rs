//! `block_allocation_algorithm` — choose disk blocks/extents.

/// Sentinel for `block_allocation_algorithm`.
pub struct BlockAllocationAlgorithm;

cast::concept! {
    name: "block_allocation_algorithm",
    summary: "choose disk blocks/extents.",
    anchors: [cast_os_stdlib::os_algorithms::block_allocation_algorithm::BlockAllocationAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
