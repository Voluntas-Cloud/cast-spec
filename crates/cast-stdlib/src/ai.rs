//! AI, agents & knowledge systems patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod agent_audit_log;
pub mod agent_capability_model;
pub mod agent_permission_boundary;
pub mod citation_requirement;
pub mod confidence_estimation;
pub mod context_isolation;
pub mod context_window_budgeting;
pub mod embedding_index;
pub mod human_approval_gate;
pub mod hybrid_search;
pub mod knowledge_graph_context;
pub mod memory_layering;
pub mod model_routing;
pub mod plan_execute_observe_loop;
pub mod prompt_injection_defense;
pub mod retrieval_augmented_generation;
pub mod semantic_cache;
pub mod source_grounding;
pub mod tool_call_contract;
pub mod tool_result_validation;

cast::concept! {
    name: "ai",
    summary: "Umbrella for the ai stdlib category. AI, agents & knowledge \
              systems patterns.",
    anchors: [
        crate::ai::agent_audit_log,
        crate::ai::agent_capability_model,
        crate::ai::agent_permission_boundary,
        crate::ai::citation_requirement,
        crate::ai::confidence_estimation,
        crate::ai::context_isolation,
        crate::ai::context_window_budgeting,
        crate::ai::embedding_index,
        crate::ai::human_approval_gate,
        crate::ai::hybrid_search,
        crate::ai::knowledge_graph_context,
        crate::ai::memory_layering,
        crate::ai::model_routing,
        crate::ai::plan_execute_observe_loop,
        crate::ai::prompt_injection_defense,
        crate::ai::retrieval_augmented_generation,
        crate::ai::semantic_cache,
        crate::ai::source_grounding,
        crate::ai::tool_call_contract,
        crate::ai::tool_result_validation,
    ],
    tags: ["cast_stdlib", "ai"],
}

/// Sentinel for the ai stdlib group.
pub struct AiGroup;

cast::rule! {
    rule: "AI agents need typed tools, scoped permissions, audit \
           trails, and boring deterministic rails.",
    why:  "Otherwise you have a chatbot with a chainsaw. Every \
           autonomous action should be traceable, every tool call \
           should be a contract, every dangerous step should pause \
           for a human.",
    governs: [cast_stdlib::ai::AiGroup],
    tags: ["cast_stdlib", "ai"],
}
