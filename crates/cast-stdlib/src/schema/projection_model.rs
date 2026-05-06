//! `projection_model` — derived/read-optimized view of source data.

/// Sentinel for `projection_model`.
pub struct ProjectionModel;

cast::concept! {
    name: "projection_model",
    summary: "Derived/read-optimized view of source data. Built for a \
              specific query pattern; updated as a function of the \
              source.",
    anchors: [cast_stdlib::schema::projection_model::ProjectionModel],
    tags: ["cast_stdlib", "schema"],
}
