//! `multilevel_feedback_queue` — dynamic queue-based scheduling.

/// Sentinel for `multilevel_feedback_queue`.
pub struct MultilevelFeedbackQueue;

cast::concept! {
    name: "multilevel_feedback_queue",
    summary: "dynamic queue-based scheduling.",
    anchors: [cast_os_stdlib::scheduling::multilevel_feedback_queue::MultilevelFeedbackQueue],
    tags: ["cast_os_stdlib", "scheduling"],
}
