//! `structured_logging` — logs as parseable fields.

/// Sentinel for `structured_logging`.
pub struct StructuredLogging;

cast::concept! {
    name: "structured_logging",
    summary: "Logs as parseable fields. Key/value or JSON, not \
              free-form prose; queryable by exact field, not by \
              regex archaeology.",
    anchors: [cast_stdlib::observability::structured_logging::StructuredLogging],
    tags: ["cast_stdlib", "observability"],
}
