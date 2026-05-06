//! `cqrs_read_write_split` — commands mutate, queries read optimized projections.

/// Sentinel for `cqrs_read_write_split`.
pub struct CqrsReadWriteSplit;

cast::concept! {
    name: "cqrs_read_write_split",
    summary: "Commands mutate state; queries read optimised \
              projections. Composes command_schema, query_schema, \
              read_model, write_model, projection_model, \
              materialized_view, eventual_projection, and \
              event_stream. Used for complex dashboards, \
              audit-backed systems, event-sourced services, \
              high-read-volume APIs, and user-facing status pages.",
    anchors: [cast_stdlib::patterns::cqrs_read_write_split::CqrsReadWriteSplit],
    tags: ["cast_stdlib", "patterns"],
}
