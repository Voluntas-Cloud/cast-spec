//! `primary_replica_model` — one writer, multiple followers.

/// Sentinel for `primary_replica_model`.
pub struct PrimaryReplicaModel;

cast::concept! {
    name: "primary_replica_model",
    summary: "One writer, multiple followers. All writes go through \
              the primary; replicas absorb read load and stand ready \
              for failover.",
    anchors: [cast_stdlib::distributed::primary_replica_model::PrimaryReplicaModel],
    tags: ["cast_stdlib", "distributed"],
}
