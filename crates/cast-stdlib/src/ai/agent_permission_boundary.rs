//! `agent_permission_boundary` — security boundary around agent actions.

/// Sentinel for `agent_permission_boundary`.
pub struct AgentPermissionBoundary;

cast::concept! {
    name: "agent_permission_boundary",
    summary: "The security boundary the agent runs inside. The agent \
              acts on behalf of someone with specific, limited \
              authority; the boundary is enforced outside the agent, \
              because asking the agent to enforce its own limits is \
              optimistic.",
    anchors: [cast_stdlib::ai::agent_permission_boundary::AgentPermissionBoundary],
    tags: ["cast_stdlib", "ai"],
}
