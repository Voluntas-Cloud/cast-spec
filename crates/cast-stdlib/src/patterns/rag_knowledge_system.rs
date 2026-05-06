//! `rag_knowledge_system` — AI answers are grounded in retrieved documents, events, or structured knowledge.

/// Sentinel for `rag_knowledge_system`.
pub struct RagKnowledgeSystem;

cast::concept! {
    name: "rag_knowledge_system",
    summary: "AI answers are grounded in retrieved documents, \
              events, or structured knowledge. Composes \
              retrieval_augmented_generation, embedding_index, \
              hybrid_search, source_grounding, citation_requirement, \
              context_window_budgeting, knowledge_graph_context, and \
              tool_result_validation. Used for project assistants, \
              documentation assistants, personal memory systems, \
              codebase assistants, and Voluntas internal AI \
              context.",
    anchors: [cast_stdlib::patterns::rag_knowledge_system::RagKnowledgeSystem],
    tags: ["cast_stdlib", "patterns"],
}
