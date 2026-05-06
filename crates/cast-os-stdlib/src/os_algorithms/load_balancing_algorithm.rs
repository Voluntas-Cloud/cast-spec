//! `load_balancing_algorithm` — distribute runnable tasks.

/// Sentinel for `load_balancing_algorithm`.
pub struct LoadBalancingAlgorithm;

cast::concept! {
    name: "load_balancing_algorithm",
    summary: "distribute runnable tasks.",
    anchors: [cast_os_stdlib::os_algorithms::load_balancing_algorithm::LoadBalancingAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
