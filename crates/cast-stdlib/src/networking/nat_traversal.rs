//! `nat_traversal` — connect through NAT constraints.

/// Sentinel for `nat_traversal`.
pub struct NatTraversal;

cast::concept! {
    name: "nat_traversal",
    summary: "Connect through NAT constraints. Hole-punching, STUN/TURN, \
              or rendezvous brokers establish flows between peers when \
              neither side has a routable address.",
    anchors: [cast_stdlib::networking::nat_traversal::NatTraversal],
    tags: ["cast_stdlib", "networking"],
}
