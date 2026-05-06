//! `io_completion_queue` — completed I/O events are queued.

/// Sentinel for `io_completion_queue`.
pub struct IoCompletionQueue;

cast::concept! {
    name: "io_completion_queue",
    summary: "completed I/O events are queued.",
    anchors: [cast_os_stdlib::io_architecture::io_completion_queue::IoCompletionQueue],
    tags: ["cast_os_stdlib", "io_architecture"],
}
