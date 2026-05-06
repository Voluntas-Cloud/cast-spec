//! `heartbeat_failure_detection` — detect failed nodes.

/// Sentinel for `heartbeat_failure_detection`.
pub struct HeartbeatFailureDetection;

cast::concept! {
    name: "heartbeat_failure_detection",
    summary: "detect failed nodes.",
    anchors: [cast_os_stdlib::distributed_os::heartbeat_failure_detection::HeartbeatFailureDetection],
    tags: ["cast_os_stdlib", "distributed_os"],
}
