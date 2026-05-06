//! `ingress_gateway` — controlled entry into cluster/network.

/// Sentinel for `ingress_gateway`.
pub struct IngressGateway;

cast::concept! {
    name: "ingress_gateway",
    summary: "Controlled entry into a cluster or network. One enforced \
              waypoint terminates external traffic; routing, authn, and \
              policy apply before requests reach internal services.",
    anchors: [cast_stdlib::networking::ingress_gateway::IngressGateway],
    tags: ["cast_stdlib", "networking"],
}
