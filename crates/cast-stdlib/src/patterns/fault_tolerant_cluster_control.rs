//! `fault_tolerant_cluster_control` — a cluster continues operating when nodes or services fail.

/// Sentinel for `fault_tolerant_cluster_control`.
pub struct FaultTolerantClusterControl;

cast::concept! {
    name: "fault_tolerant_cluster_control",
    summary: "A cluster continues operating when nodes or services \
              fail. Composes heartbeat_signal, failure_detector, \
              leader_election, quorum_decision, failover, \
              split_brain_prevention, checkpoint_resume, and \
              disaster_recovery_plan. Used for multi-node clusters, \
              control-plane HA, storage failover, distributed service \
              management, and home-cloud resilience.",
    anchors: [cast_stdlib::patterns::fault_tolerant_cluster_control::FaultTolerantClusterControl],
    tags: ["cast_stdlib", "patterns"],
}
