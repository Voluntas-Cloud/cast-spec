//! `io_scheduler` — orders storage I/O requests.

/// Sentinel for `io_scheduler`.
pub struct IoScheduler;

cast::concept! {
    name: "io_scheduler",
    summary: "orders storage I/O requests.",
    anchors: [cast_os_stdlib::io_architecture::io_scheduler::IoScheduler],
    tags: ["cast_os_stdlib", "io_architecture"],
}
