//! `nonblocking_io` — operation returns immediately if not ready.

/// Sentinel for `nonblocking_io`.
pub struct NonblockingIo;

cast::concept! {
    name: "nonblocking_io",
    summary: "operation returns immediately if not ready.",
    anchors: [cast_os_stdlib::io_architecture::nonblocking_io::NonblockingIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
