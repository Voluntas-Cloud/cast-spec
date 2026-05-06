//! `network_policy` — explicit allowed traffic.

/// Sentinel for `network_policy`.
pub struct NetworkPolicy;

cast::concept! {
    name: "network_policy",
    summary: "Explicit allowed traffic. The set of permitted \
              source/destination/port tuples is declared and enforced; \
              everything else is denied at the network layer rather than \
              the application.",
    anchors: [cast_stdlib::networking::network_policy::NetworkPolicy],
    tags: ["cast_stdlib", "networking"],
}
