//! `throughput_limit` — max work per time unit.

/// Sentinel for `throughput_limit`.
pub struct ThroughputLimit;

cast::concept! {
    name: "throughput_limit",
    summary: "Max work per time unit. The ceiling beyond which the \
              system cannot keep up; informs autoscaling triggers and \
              capacity plans.",
    anchors: [cast_stdlib::performance::throughput_limit::ThroughputLimit],
    tags: ["cast_stdlib", "performance"],
}
