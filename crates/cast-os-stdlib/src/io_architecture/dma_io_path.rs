//! `dma_io_path` — device transfers memory directly.

/// Sentinel for `dma_io_path`.
pub struct DmaIoPath;

cast::concept! {
    name: "dma_io_path",
    summary: "device transfers memory directly.",
    anchors: [cast_os_stdlib::io_architecture::dma_io_path::DmaIoPath],
    tags: ["cast_os_stdlib", "io_architecture"],
}
