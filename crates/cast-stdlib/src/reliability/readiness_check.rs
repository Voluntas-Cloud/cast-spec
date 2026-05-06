//! `readiness_check` — determine if a service can receive traffic.

/// Sentinel for `readiness_check`.
pub struct ReadinessCheck;

cast::concept! {
    name: "readiness_check",
    summary: "Determine if service can receive traffic. Failing means \
              traffic is routed elsewhere; the process keeps running, \
              the load balancer just stops sending requests.",
    anchors: [cast_stdlib::reliability::readiness_check::ReadinessCheck],
    tags: ["cast_stdlib", "reliability"],
}
