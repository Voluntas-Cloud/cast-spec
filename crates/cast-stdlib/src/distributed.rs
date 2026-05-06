//! Distributed systems & coordination patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod clock_skew_handling;
pub mod consensus_protocol;
pub mod distributed_barrier;
pub mod distributed_checkpoint;
pub mod failure_detector;
pub mod fencing_token;
pub mod gossip_protocol;
pub mod heartbeat_signal;
pub mod leader_election;
pub mod lease_ownership;
pub mod logical_clock;
pub mod membership_protocol;
pub mod multi_primary_model;
pub mod partition_tolerance;
pub mod primary_replica_model;
pub mod quorum_read_write;
pub mod read_replica;
pub mod replication_log;
pub mod split_brain_recovery;
pub mod vector_clock;

cast::concept! {
    name: "distributed",
    summary: "Umbrella for the distributed stdlib category. Distributed \
              systems & coordination patterns.",
    anchors: [
        crate::distributed::clock_skew_handling,
        crate::distributed::consensus_protocol,
        crate::distributed::distributed_barrier,
        crate::distributed::distributed_checkpoint,
        crate::distributed::failure_detector,
        crate::distributed::fencing_token,
        crate::distributed::gossip_protocol,
        crate::distributed::heartbeat_signal,
        crate::distributed::leader_election,
        crate::distributed::lease_ownership,
        crate::distributed::logical_clock,
        crate::distributed::membership_protocol,
        crate::distributed::multi_primary_model,
        crate::distributed::partition_tolerance,
        crate::distributed::primary_replica_model,
        crate::distributed::quorum_read_write,
        crate::distributed::read_replica,
        crate::distributed::replication_log,
        crate::distributed::split_brain_recovery,
        crate::distributed::vector_clock,
    ],
    tags: ["cast_stdlib", "distributed"],
}

/// Sentinel for the distributed stdlib group.
pub struct DistributedGroup;

cast::rule! {
    rule: "Use boring, proven coordination systems.",
    why:  "'We can make our own consensus' is how engineers summon \
           demons with YAML. Distributed coordination is a graveyard \
           of carefully-reasoned designs that survived the whiteboard \
           and not the partition.",
    governs: [cast_stdlib::distributed::DistributedGroup],
    tags: ["cast_stdlib", "distributed"],
}
