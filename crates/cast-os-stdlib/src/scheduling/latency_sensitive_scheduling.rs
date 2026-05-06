//! `latency_sensitive_scheduling` — optimize response time.

/// Sentinel for `latency_sensitive_scheduling`.
pub struct LatencySensitiveScheduling;

cast::concept! {
    name: "latency_sensitive_scheduling",
    summary: "optimize response time.",
    anchors: [cast_os_stdlib::scheduling::latency_sensitive_scheduling::LatencySensitiveScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
