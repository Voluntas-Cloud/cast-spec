//! `cluster_recovery_protocol` — restore safe operation after failure.

/// Sentinel for `cluster_recovery_protocol`.
pub struct ClusterRecoveryProtocol;

cast::concept! {
    name: "cluster_recovery_protocol",
    summary: "restore safe operation after failure.",
    anchors: [cast_os_stdlib::distributed_os::cluster_recovery_protocol::ClusterRecoveryProtocol],
    tags: ["cast_os_stdlib", "distributed_os"],
}
