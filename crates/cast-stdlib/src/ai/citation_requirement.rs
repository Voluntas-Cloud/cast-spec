//! `citation_requirement` — require references for factual claims.

/// Sentinel for `citation_requirement`.
pub struct CitationRequirement;

cast::concept! {
    name: "citation_requirement",
    summary: "Factual claims must come with a reference the user can \
              follow. Citations are the difference between a research \
              tool and a confident parrot, and they're the cheapest \
              defence against hallucination available.",
    anchors: [cast_stdlib::ai::citation_requirement::CitationRequirement],
    tags: ["cast_stdlib", "ai"],
}
