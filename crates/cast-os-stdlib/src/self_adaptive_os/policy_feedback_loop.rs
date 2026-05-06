//! `policy_feedback_loop` — observe, decide, act, evaluate.

/// Sentinel for `policy_feedback_loop`.
pub struct PolicyFeedbackLoop;

cast::concept! {
    name: "policy_feedback_loop",
    summary: "observe, decide, act, evaluate.",
    anchors: [cast_os_stdlib::self_adaptive_os::policy_feedback_loop::PolicyFeedbackLoop],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
