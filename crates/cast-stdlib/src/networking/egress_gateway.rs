//! `egress_gateway` — controlled exit from cluster/network.

/// Sentinel for `egress_gateway`.
pub struct EgressGateway;

cast::concept! {
    name: "egress_gateway",
    summary: "Controlled exit from cluster or network. Outbound traffic \
              is funnelled through a known waypoint so leaks are visible \
              and policy can hold over what services may call out to.",
    anchors: [cast_stdlib::networking::egress_gateway::EgressGateway],
    tags: ["cast_stdlib", "networking"],
}
