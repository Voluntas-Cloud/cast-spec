//! `defense_in_depth` — multiple layers of protection.

/// Sentinel for `defense_in_depth`.
pub struct DefenseInDepth;

cast::concept! {
    name: "defense_in_depth",
    summary: "Multiple layers of protection. No single control is the \
              last line; auth, encryption, network policy, and audit \
              each constrain the blast radius of the others' failures.",
    anchors: [cast_stdlib::security::defense_in_depth::DefenseInDepth],
    tags: ["cast_stdlib", "security"],
}
