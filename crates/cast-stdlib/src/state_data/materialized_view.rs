//! `materialized_view` — stored derived query result.

/// Sentinel for `materialized_view`.
pub struct MaterializedView;

cast::concept! {
    name: "materialized_view",
    summary: "A derived query result stored for cheap reads. The \
              database keeps it in sync; you keep its definition in \
              source control. Running a materialised view whose \
              definition lives only in someone's head is a slow \
              corruption.",
    anchors: [cast_stdlib::state_data::materialized_view::MaterializedView],
    tags: ["cast_stdlib", "state_data"],
}
