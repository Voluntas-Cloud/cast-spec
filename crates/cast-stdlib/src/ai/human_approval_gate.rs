//! `human_approval_gate` — agent pauses before risky action.

/// Sentinel for `human_approval_gate`.
pub struct HumanApprovalGate;

cast::concept! {
    name: "human_approval_gate",
    summary: "Before a risky action — destructive, irreversible, \
              expensive — the agent waits for a human. Place the gate \
              before the action, not after; \"sorry, I sent the \
              email\" is not recovery.",
    anchors: [cast_stdlib::ai::human_approval_gate::HumanApprovalGate],
    tags: ["cast_stdlib", "ai"],
}
