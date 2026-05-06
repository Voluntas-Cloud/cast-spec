//! `knowledge_graph_context` — model entities and relations.

/// Sentinel for `knowledge_graph_context`.
pub struct KnowledgeGraphContext;

cast::concept! {
    name: "knowledge_graph_context",
    summary: "Represent context as entities and relations rather than \
              a flat blob of prose. The graph lets the system answer \
              \"what is connected to X\" without re-summarising the \
              corpus every time.",
    anchors: [cast_stdlib::ai::knowledge_graph_context::KnowledgeGraphContext],
    tags: ["cast_stdlib", "ai"],
}
