//! `load_balancer` — distribute traffic across backends.

/// Sentinel for `load_balancer`.
pub struct LoadBalancer;

cast::concept! {
    name: "load_balancer",
    summary: "Distribute traffic across backends. The selection algorithm \
              (round-robin, least-conn, hash) and health-check loop \
              determine fairness, locality, and how fast bad backends \
              are shed.",
    anchors: [cast_stdlib::networking::load_balancer::LoadBalancer],
    tags: ["cast_stdlib", "networking"],
}
