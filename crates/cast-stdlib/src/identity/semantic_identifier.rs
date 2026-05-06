//! `semantic_identifier` — ID encoding domain meaning.

/// Sentinel for `semantic_identifier`.
pub struct SemanticIdentifier;

cast::concept! {
    name: "semantic_identifier",
    summary: "ID that carries domain meaning, e.g. invoice-2026-001. \
              Convenient for humans; brittle to domain changes \
              (renaming, merging, year rollover).",
    anchors: [cast_stdlib::identity::semantic_identifier::SemanticIdentifier],
    tags: ["cast_stdlib", "identity"],
}
