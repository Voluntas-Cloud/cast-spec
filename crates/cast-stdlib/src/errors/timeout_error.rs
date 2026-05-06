//! `timeout_error` — operation exceeded time budget.

/// Sentinel for `timeout_error`.
pub struct TimeoutError;

cast::concept! {
    name: "timeout_error",
    summary: "Operation exceeded time budget. The caller stopped \
              waiting; the work may still be in flight. Idempotency or \
              an explicit cancel is what makes timeout a real outcome \
              instead of an undefined one.",
    anchors: [cast_stdlib::errors::timeout_error::TimeoutError],
    tags: ["cast_stdlib", "errors"],
}
