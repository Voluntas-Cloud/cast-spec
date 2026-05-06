//! `dma_engine` — driver mechanism for direct memory access.

/// Sentinel for `dma_engine`.
pub struct DmaEngine;

cast::concept! {
    name: "dma_engine",
    summary: "driver mechanism for direct memory access.",
    anchors: [cast_os_stdlib::driver_model::dma_engine::DmaEngine],
    tags: ["cast_os_stdlib", "driver_model"],
}
