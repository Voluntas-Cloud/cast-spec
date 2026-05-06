//! `autoscaling` — adjust capacity automatically.

/// Sentinel for `autoscaling`.
pub struct Autoscaling;

cast::concept! {
    name: "autoscaling",
    summary: "Adjust capacity automatically. Replicas grow under load \
              and shrink when idle; the policy and the metrics it \
              reads are load-bearing.",
    anchors: [cast_stdlib::performance::autoscaling::Autoscaling],
    tags: ["cast_stdlib", "performance"],
}
