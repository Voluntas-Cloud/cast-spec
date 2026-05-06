//! `embedding_index` — vector representation for semantic lookup.

/// Sentinel for `embedding_index`.
pub struct EmbeddingIndex;

cast::concept! {
    name: "embedding_index",
    summary: "Vector representation of content used for nearest- \
              neighbour lookup. The index has a model version, a \
              chunking strategy, and a refresh story — none of which \
              are optional, even though most tutorials skip them.",
    anchors: [cast_stdlib::ai::embedding_index::EmbeddingIndex],
    tags: ["cast_stdlib", "ai"],
}
