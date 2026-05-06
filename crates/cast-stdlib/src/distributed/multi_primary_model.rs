//! `multi_primary_model` — multiple writers with conflict handling.

/// Sentinel for `multi_primary_model`.
pub struct MultiPrimaryModel;

cast::concept! {
    name: "multi_primary_model",
    summary: "Multiple writers with conflict handling. Higher write \
              availability than single-primary; pays for it in \
              conflict resolution complexity.",
    anchors: [cast_stdlib::distributed::multi_primary_model::MultiPrimaryModel],
    tags: ["cast_stdlib", "distributed"],
}
