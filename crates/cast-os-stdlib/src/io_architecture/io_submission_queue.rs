//! `io_submission_queue` — requested I/O operations are queued.

/// Sentinel for `io_submission_queue`.
pub struct IoSubmissionQueue;

cast::concept! {
    name: "io_submission_queue",
    summary: "requested I/O operations are queued.",
    anchors: [cast_os_stdlib::io_architecture::io_submission_queue::IoSubmissionQueue],
    tags: ["cast_os_stdlib", "io_architecture"],
}
