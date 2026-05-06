//! `recoverable_error` — caller can retry or handle.

/// Sentinel for `recoverable_error`.
pub struct RecoverableError;

cast::concept! {
    name: "recoverable_error",
    summary: "Caller can retry or handle. The error is transient or \
              addressable in-context; the API distinguishes it from \
              fatal failures so retry loops don't loop forever on \
              hopeless cases.",
    anchors: [cast_stdlib::errors::recoverable_error::RecoverableError],
    tags: ["cast_stdlib", "errors"],
}
