//! `dma_buffer_mapping` — map memory for device access.

/// Sentinel for `dma_buffer_mapping`.
pub struct DmaBufferMapping;

cast::concept! {
    name: "dma_buffer_mapping",
    summary: "map memory for device access.",
    anchors: [cast_os_stdlib::memory_management::dma_buffer_mapping::DmaBufferMapping],
    tags: ["cast_os_stdlib", "memory_management"],
}
