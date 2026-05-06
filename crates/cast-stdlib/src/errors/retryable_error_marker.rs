//! `retryable_error_marker` — machine-readable retry hint.

/// Sentinel for `retryable_error_marker`.
pub struct RetryableErrorMarker;

cast::concept! {
    name: "retryable_error_marker",
    summary: "Machine-readable retry hint. The error carries a flag (or \
              a category callers map to one) that says \"safe to retry \
              after backoff\"; clients don't have to guess from prose \
              or status codes.",
    anchors: [cast_stdlib::errors::retryable_error_marker::RetryableErrorMarker],
    tags: ["cast_stdlib", "errors"],
}
