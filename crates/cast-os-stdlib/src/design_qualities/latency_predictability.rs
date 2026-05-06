//! `latency_predictability` — bounded response time.

/// Sentinel for `latency_predictability`.
pub struct LatencyPredictability;

cast::concept! {
    name: "latency_predictability",
    summary: "bounded response time.",
    anchors: [cast_os_stdlib::design_qualities::latency_predictability::LatencyPredictability],
    tags: ["cast_os_stdlib", "design_qualities"],
}
