//! `drift_detection` — divergence between desired and actual.

/// Sentinel for `drift_detection`.
pub struct DriftDetection;

cast::concept! {
    name: "drift_detection",
    summary: "Detect divergence between desired and actual state. \
              Alerts, dashboards, or auto-remediation depending on \
              the impact and reversibility of the drift.",
    anchors: [cast_stdlib::lifecycle::drift_detection::DriftDetection],
    tags: ["cast_stdlib", "lifecycle"],
}
