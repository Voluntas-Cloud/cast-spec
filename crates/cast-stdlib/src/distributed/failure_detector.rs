//! `failure_detector` — infer failed nodes.

/// Sentinel for `failure_detector`.
pub struct FailureDetector;

cast::concept! {
    name: "failure_detector",
    summary: "Infer failed nodes. Heartbeats and timeouts can never be \
              perfect; the detector exposes a tunable tradeoff between \
              false positives and detection latency.",
    anchors: [cast_stdlib::distributed::failure_detector::FailureDetector],
    tags: ["cast_stdlib", "distributed"],
}
