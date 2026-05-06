//! `source_grounding` — tie claims to evidence.

/// Sentinel for `source_grounding`.
pub struct SourceGrounding;

cast::concept! {
    name: "source_grounding",
    summary: "Tie claims back to the evidence they came from. Without \
              grounding, the model is allowed to be confidently wrong; \
              with it, the user can verify and the system can detect \
              when the source no longer supports the claim.",
    anchors: [cast_stdlib::ai::source_grounding::SourceGrounding],
    tags: ["cast_stdlib", "ai"],
}
