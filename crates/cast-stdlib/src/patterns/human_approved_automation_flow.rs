//! `human_approved_automation_flow` — automation prepares actions; execution requires human approval.

/// Sentinel for `human_approved_automation_flow`.
pub struct HumanApprovedAutomationFlow;

cast::concept! {
    name: "human_approved_automation_flow",
    summary: "Automation may propose or prepare actions, but \
              execution requires human approval. Composes \
              human_approval_step, signed_request, user_intent_model, \
              permission_prompt, action_preview, audit_log, \
              capability_token, and replay_guard. Used for AI agents \
              performing privileged actions, payment approvals, \
              infrastructure changes, privacy-sensitive operations, \
              and mobile signing flows.",
    anchors: [cast_stdlib::patterns::human_approved_automation_flow::HumanApprovedAutomationFlow],
    tags: ["cast_stdlib", "patterns"],
}
