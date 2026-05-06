//! `scatter_gather_io` — I/O across multiple buffers.

/// Sentinel for `scatter_gather_io`.
pub struct ScatterGatherIo;

cast::concept! {
    name: "scatter_gather_io",
    summary: "I/O across multiple buffers.",
    anchors: [cast_os_stdlib::io_architecture::scatter_gather_io::ScatterGatherIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
