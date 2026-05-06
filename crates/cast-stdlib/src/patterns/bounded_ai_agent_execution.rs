//! `bounded_ai_agent_execution` — agents act through typed tools, scoped permissions, and audit.

/// Sentinel for `bounded_ai_agent_execution`.
pub struct BoundedAiAgentExecution;

cast::concept! {
    name: "bounded_ai_agent_execution",
    summary: "AI agents can act, but only through typed tools, \
              scoped permissions, and audit trails. Composes \
              tool_call_contract, agent_capability_model, \
              agent_permission_boundary, human_approval_gate, \
              audit_log, policy_as_code, prompt_injection_defense, \
              and context_isolation. Used for AI ops assistants, \
              coding agents, personal admin assistants, cluster \
              automation, and agent-based workflow execution.",
    anchors: [cast_stdlib::patterns::bounded_ai_agent_execution::BoundedAiAgentExecution],
    tags: ["cast_stdlib", "patterns"],
}
