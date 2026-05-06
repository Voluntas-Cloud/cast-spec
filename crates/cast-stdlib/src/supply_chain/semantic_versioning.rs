//! `semantic_versioning` — version numbers communicate compatibility.

/// Sentinel for `semantic_versioning`.
pub struct SemanticVersioning;

cast::concept! {
    name: "semantic_versioning",
    summary: "Version numbers communicate compatibility. Major = \
              breaking, minor = additive, patch = cosmetic; consumers \
              can decide whether to take an upgrade by reading the \
              version delta.",
    anchors: [cast_stdlib::supply_chain::semantic_versioning::SemanticVersioning],
    tags: ["cast_stdlib", "supply_chain"],
}
