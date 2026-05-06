//! `replication_log` — ordered record copied to replicas.

/// Sentinel for `replication_log`.
pub struct ReplicationLog;

cast::concept! {
    name: "replication_log",
    summary: "Ordered record copied to replicas. Replicas apply log \
              entries in order; combined with consensus, the substrate \
              of state-machine replication.",
    anchors: [cast_stdlib::distributed::replication_log::ReplicationLog],
    tags: ["cast_stdlib", "distributed"],
}
