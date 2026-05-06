//! `leader_election_service` — choose cluster authority.

/// Sentinel for `leader_election_service`.
pub struct LeaderElectionService;

cast::concept! {
    name: "leader_election_service",
    summary: "choose cluster authority.",
    anchors: [cast_os_stdlib::distributed_os::leader_election_service::LeaderElectionService],
    tags: ["cast_os_stdlib", "distributed_os"],
}
