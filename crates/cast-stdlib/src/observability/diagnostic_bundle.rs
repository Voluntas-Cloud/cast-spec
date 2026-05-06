//! `diagnostic_bundle` — packaged logs/config/state for debugging.

/// Sentinel for `diagnostic_bundle`.
pub struct DiagnosticBundle;

cast::concept! {
    name: "diagnostic_bundle",
    summary: "Packaged logs/config/state for debugging. One artifact \
              an operator can hand to support that contains everything \
              needed to start investigating.",
    anchors: [cast_stdlib::observability::diagnostic_bundle::DiagnosticBundle],
    tags: ["cast_stdlib", "observability"],
}
