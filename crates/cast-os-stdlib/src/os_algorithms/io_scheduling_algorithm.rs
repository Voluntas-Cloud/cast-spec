//! `io_scheduling_algorithm` — order storage operations.

/// Sentinel for `io_scheduling_algorithm`.
pub struct IoSchedulingAlgorithm;

cast::concept! {
    name: "io_scheduling_algorithm",
    summary: "order storage operations.",
    anchors: [cast_os_stdlib::os_algorithms::io_scheduling_algorithm::IoSchedulingAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
