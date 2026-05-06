//! `filtering_contract` — structured query constraints.

/// Sentinel for `filtering_contract`.
pub struct FilteringContract;

cast::concept! {
    name: "filtering_contract",
    summary: "Structured query constraints. Callers express intent in a \
              defined grammar; servers do not parse free-form text.",
    anchors: [cast_stdlib::api::filtering_contract::FilteringContract],
    tags: ["cast_stdlib", "api"],
}
