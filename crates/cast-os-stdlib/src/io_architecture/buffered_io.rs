//! `buffered_io` — use OS cache.

/// Sentinel for `buffered_io`.
pub struct BufferedIo;

cast::concept! {
    name: "buffered_io",
    summary: "use OS cache.",
    anchors: [cast_os_stdlib::io_architecture::buffered_io::BufferedIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
