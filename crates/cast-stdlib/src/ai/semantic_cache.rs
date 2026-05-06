//! `semantic_cache` — reuse prior AI responses based on meaning.

/// Sentinel for `semantic_cache`.
pub struct SemanticCache;

cast::concept! {
    name: "semantic_cache",
    summary: "Reuse a prior response when the new request means the \
              same thing as an old one. Saves money and latency, but \
              the similarity threshold is the load-bearing parameter \
              — too loose and you serve confidently wrong answers.",
    anchors: [cast_stdlib::ai::semantic_cache::SemanticCache],
    tags: ["cast_stdlib", "ai"],
}
