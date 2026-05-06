//! `error_redaction` — hide sensitive details.

/// Sentinel for `error_redaction`.
pub struct ErrorRedaction;

cast::concept! {
    name: "error_redaction",
    summary: "Hide sensitive details before showing an error to \
              outsiders. Internal logs keep the full chain; external \
              responses lose secrets, paths, and SQL fragments. \
              Redaction happens at presentation, not at write.",
    anchors: [cast_stdlib::errors::error_redaction::ErrorRedaction],
    tags: ["cast_stdlib", "errors"],
}
