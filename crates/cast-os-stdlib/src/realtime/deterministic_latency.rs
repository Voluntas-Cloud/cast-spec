//! `deterministic_latency` — bounded execution response time.

/// Sentinel for `deterministic_latency`.
pub struct DeterministicLatency;

cast::concept! {
    name: "deterministic_latency",
    summary: "bounded execution response time.",
    anchors: [cast_os_stdlib::realtime::deterministic_latency::DeterministicLatency],
    tags: ["cast_os_stdlib", "realtime"],
}
