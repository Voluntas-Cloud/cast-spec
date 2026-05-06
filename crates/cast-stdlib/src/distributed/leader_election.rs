//! `leader_election` — choose one coordinator.

/// Sentinel for `leader_election`.
pub struct LeaderElection;

cast::concept! {
    name: "leader_election",
    summary: "Choose one coordinator. The cluster picks a single node \
              to make decisions; the protocol guarantees at most one \
              winner even under partitions.",
    anchors: [cast_stdlib::distributed::leader_election::LeaderElection],
    tags: ["cast_stdlib", "distributed"],
}
