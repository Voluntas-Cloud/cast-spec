//! `linux_io_uring_model` — shared ring-based async I/O mechanism.

/// Sentinel for `linux_io_uring_model`.
pub struct LinuxIoUringModel;

cast::concept! {
    name: "linux_io_uring_model",
    summary: "shared ring-based async I/O mechanism.",
    anchors: [cast_os_stdlib::io_architecture::linux_io_uring_model::LinuxIoUringModel],
    tags: ["cast_os_stdlib", "io_architecture"],
}
