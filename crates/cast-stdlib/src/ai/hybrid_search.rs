//! `hybrid_search` — combine keyword and semantic retrieval.

/// Sentinel for `hybrid_search`.
pub struct HybridSearch;

cast::concept! {
    name: "hybrid_search",
    summary: "Combine keyword and semantic retrieval. Keyword search \
              wins on exact identifiers and proper nouns; embeddings \
              win on paraphrase. The hybrid covers the embarrassing \
              cases neither does well alone.",
    anchors: [cast_stdlib::ai::hybrid_search::HybridSearch],
    tags: ["cast_stdlib", "ai"],
}
