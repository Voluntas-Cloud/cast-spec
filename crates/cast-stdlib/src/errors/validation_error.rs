//! `validation_error` — input violates contract.

/// Sentinel for `validation_error`.
pub struct ValidationError;

cast::concept! {
    name: "validation_error",
    summary: "Input violates contract. The caller's request is malformed \
              by definition; no amount of retry helps. The shape of the \
              response should tell the caller exactly which field is \
              wrong.",
    anchors: [cast_stdlib::errors::validation_error::ValidationError],
    tags: ["cast_stdlib", "errors"],
}
