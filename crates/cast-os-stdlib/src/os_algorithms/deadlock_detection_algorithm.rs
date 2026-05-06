//! `deadlock_detection_algorithm` — find cycles in lock/resource waits.

/// Sentinel for `deadlock_detection_algorithm`.
pub struct DeadlockDetectionAlgorithm;

cast::concept! {
    name: "deadlock_detection_algorithm",
    summary: "find cycles in lock/resource waits.",
    anchors: [cast_os_stdlib::os_algorithms::deadlock_detection_algorithm::DeadlockDetectionAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
