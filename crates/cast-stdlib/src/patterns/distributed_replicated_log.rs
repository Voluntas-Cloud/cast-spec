//! `distributed_replicated_log` — nodes agree on ordered state changes through replication.

/// Sentinel for `distributed_replicated_log`.
pub struct DistributedReplicatedLog;

cast::concept! {
    name: "distributed_replicated_log",
    summary: "Nodes agree on ordered state changes through \
              replication. Composes replication_log, \
              consensus_protocol, quorum_read_write, \
              monotonic_sequence_id, snapshot_storage, \
              leader_election, and split_brain_prevention. Used for \
              metadata stores, cluster state, distributed databases, \
              control-plane coordination, and configuration history.",
    anchors: [cast_stdlib::patterns::distributed_replicated_log::DistributedReplicatedLog],
    tags: ["cast_stdlib", "patterns"],
}
