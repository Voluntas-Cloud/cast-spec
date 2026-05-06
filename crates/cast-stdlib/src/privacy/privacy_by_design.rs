//! `privacy_by_design` — privacy built into architecture.

/// Sentinel for `privacy_by_design`.
pub struct PrivacyByDesign;

cast::concept! {
    name: "privacy_by_design",
    summary: "Privacy built into the architecture. Minimisation, \
              purpose, and consent shape the schema and data flow from \
              day one; bolting privacy on later requires rewriting the \
              parts you already shipped.",
    anchors: [cast_stdlib::privacy::privacy_by_design::PrivacyByDesign],
    tags: ["cast_stdlib", "privacy"],
}
