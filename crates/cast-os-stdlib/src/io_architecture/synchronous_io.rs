//! `synchronous_io` — caller waits for I/O completion.

/// Sentinel for `synchronous_io`.
pub struct SynchronousIo;

cast::concept! {
    name: "synchronous_io",
    summary: "caller waits for I/O completion.",
    anchors: [cast_os_stdlib::io_architecture::synchronous_io::SynchronousIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
