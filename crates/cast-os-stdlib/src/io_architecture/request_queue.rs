//! `request_queue` — pending I/O operation structure.

/// Sentinel for `request_queue`.
pub struct RequestQueue;

cast::concept! {
    name: "request_queue",
    summary: "pending I/O operation structure.",
    anchors: [cast_os_stdlib::io_architecture::request_queue::RequestQueue],
    tags: ["cast_os_stdlib", "io_architecture"],
}
