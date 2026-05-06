//! `chaos_testing` — intentionally inject failure.

/// Sentinel for `chaos_testing`.
pub struct ChaosTesting;

cast::concept! {
    name: "chaos_testing",
    summary: "Intentionally inject failure. Latency, packet loss, \
              process kills, partition — under controlled conditions \
              so the response is observable before it happens for real.",
    anchors: [cast_stdlib::reliability::chaos_testing::ChaosTesting],
    tags: ["cast_stdlib", "reliability"],
}
