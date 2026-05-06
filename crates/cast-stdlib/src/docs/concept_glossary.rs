//! `concept_glossary` — shared definitions.

/// Sentinel for `concept_glossary`.
pub struct ConceptGlossary;

cast::concept! {
    name: "concept_glossary",
    summary: "Shared definitions for terms the team uses. Without one, \
              every conversation rederives what \"tenant\" or \
              \"intent\" means and quietly disagrees about it.",
    anchors: [cast_stdlib::docs::concept_glossary::ConceptGlossary],
    tags: ["cast_stdlib", "docs"],
}
