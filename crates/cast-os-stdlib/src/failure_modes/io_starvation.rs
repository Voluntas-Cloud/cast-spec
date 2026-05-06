//! `io_starvation` — I/O request waits too long.

/// Sentinel for `io_starvation`.
pub struct IoStarvation;

cast::concept! {
    name: "io_starvation",
    summary: "I/O request waits too long.",
    anchors: [cast_os_stdlib::failure_modes::io_starvation::IoStarvation],
    tags: ["cast_os_stdlib", "failure_modes"],
}
