//! `temporal_query` — query state as of a time/version.

/// Sentinel for `temporal_query`.
pub struct TemporalQuery;

cast::concept! {
    name: "temporal_query",
    summary: "Ask the system what state looked like at a previous \
              point. Indispensable for audits, reproductions, and \
              \"why did the report show that\". Requires the storage \
              layer to keep history rather than overwriting in place.",
    anchors: [cast_stdlib::time_ordering::temporal_query::TemporalQuery],
    tags: ["cast_stdlib", "time_ordering"],
}
