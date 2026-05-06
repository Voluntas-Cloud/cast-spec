//! `causal_consistency` — cause/effect ordering preserved.

/// Sentinel for `causal_consistency`.
pub struct CausalConsistency;

cast::concept! {
    name: "causal_consistency",
    summary: "Cause/effect ordering is preserved. Effects appear after \
              their causes; unrelated operations may be reordered.",
    anchors: [cast_stdlib::consistency::causal_consistency::CausalConsistency],
    tags: ["cast_stdlib", "consistency"],
}
