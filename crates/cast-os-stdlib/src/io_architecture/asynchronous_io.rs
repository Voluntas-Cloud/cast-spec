//! `asynchronous_io` — caller continues while I/O proceeds.

/// Sentinel for `asynchronous_io`.
pub struct AsynchronousIo;

cast::concept! {
    name: "asynchronous_io",
    summary: "caller continues while I/O proceeds.",
    anchors: [cast_os_stdlib::io_architecture::asynchronous_io::AsynchronousIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
