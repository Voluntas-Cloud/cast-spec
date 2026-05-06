//! `operator_facing_error` — diagnostic detail for maintainers.

/// Sentinel for `operator_facing_error`.
pub struct OperatorFacingError;

cast::concept! {
    name: "operator_facing_error",
    summary: "Diagnostic detail for maintainers. Stack traces, request \
              IDs, internal state — everything required to debug — \
              kept on the operator side of the boundary, never returned \
              to clients.",
    anchors: [cast_stdlib::errors::operator_facing_error::OperatorFacingError],
    tags: ["cast_stdlib", "errors"],
}
