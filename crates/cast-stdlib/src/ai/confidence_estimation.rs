//! `confidence_estimation` — expose uncertainty/use evidence thresholds.

/// Sentinel for `confidence_estimation`.
pub struct ConfidenceEstimation;

cast::concept! {
    name: "confidence_estimation",
    summary: "Expose how confident the system is in an answer, and \
              gate behaviour on that confidence. \"I don't know\" is \
              a feature; pretending to know is the failure mode users \
              learn to spot and stop trusting.",
    anchors: [cast_stdlib::ai::confidence_estimation::ConfidenceEstimation],
    tags: ["cast_stdlib", "ai"],
}
