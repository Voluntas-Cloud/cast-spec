//! `agent_capability_model` — explicit list of what an agent may do.

/// Sentinel for `agent_capability_model`.
pub struct AgentCapabilityModel;

cast::concept! {
    name: "agent_capability_model",
    summary: "An explicit list of what the agent is permitted to do. \
              The capability model is what auditors and users read; \
              \"it can do anything its tools let it do\" is not a \
              security model, it's a confession.",
    anchors: [cast_stdlib::ai::agent_capability_model::AgentCapabilityModel],
    tags: ["cast_stdlib", "ai"],
}
