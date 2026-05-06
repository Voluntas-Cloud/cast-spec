//! `latency_budget` — allocate acceptable delay across steps.

/// Sentinel for `latency_budget`.
pub struct LatencyBudget;

cast::concept! {
    name: "latency_budget",
    summary: "Allocate acceptable delay across steps. The end-to-end \
              budget gets divided among components; each component \
              knows the slice it must hit.",
    anchors: [cast_stdlib::performance::latency_budget::LatencyBudget],
    tags: ["cast_stdlib", "performance"],
}
