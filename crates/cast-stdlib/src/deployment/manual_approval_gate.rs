//! `manual_approval_gate` — human approval before risky stage.

/// Sentinel for `manual_approval_gate`.
pub struct ManualApprovalGate;

cast::concept! {
    name: "manual_approval_gate",
    summary: "Human approval before risky stage. The pipeline pauses \
              and an authorized human says 'continue'; reserved for \
              changes the automation cannot risk-assess on its own.",
    anchors: [cast_stdlib::deployment::manual_approval_gate::ManualApprovalGate],
    tags: ["cast_stdlib", "deployment"],
}
