//! `consensus_protocol` — nodes agree on state/order.

/// Sentinel for `consensus_protocol`.
pub struct ConsensusProtocol;

cast::concept! {
    name: "consensus_protocol",
    summary: "Nodes agree on state/order. Paxos, Raft, and friends; \
              the cluster reaches a single answer despite faults, at \
              the cost of latency and a quorum requirement.",
    anchors: [cast_stdlib::distributed::consensus_protocol::ConsensusProtocol],
    tags: ["cast_stdlib", "distributed"],
}
