//! `agent_audit_log` — record AI actions and rationale.

/// Sentinel for `agent_audit_log`.
pub struct AgentAuditLog;

cast::concept! {
    name: "agent_audit_log",
    summary: "Record the agent's actions and the reasoning it gave \
              for them. The log is what lets a human reconstruct what \
              happened and why; without it, debugging an autonomous \
              system is divination.",
    anchors: [cast_stdlib::ai::agent_audit_log::AgentAuditLog],
    tags: ["cast_stdlib", "ai"],
}
