//! `gossip_protocol` — nodes spread state peer-to-peer.

/// Sentinel for `gossip_protocol`.
pub struct GossipProtocol;

cast::concept! {
    name: "gossip_protocol",
    summary: "Nodes spread state peer-to-peer. Each node periodically \
              tells a few neighbors what it knows; convergence is \
              probabilistic but resilient to failure.",
    anchors: [cast_stdlib::distributed::gossip_protocol::GossipProtocol],
    tags: ["cast_stdlib", "distributed"],
}
