//! `human_approval_step` — workflow pauses for human decision.

/// Sentinel for `human_approval_step`.
pub struct HumanApprovalStep;

cast::concept! {
    name: "human_approval_step",
    summary: "Workflow pauses for human decision. A first-class step \
              type — not a sleep loop polling a flag — with an explicit \
              approver, deadline, and what happens on timeout or denial.",
    anchors: [cast_stdlib::workflow::human_approval_step::HumanApprovalStep],
    tags: ["cast_stdlib", "workflow"],
}
