//! `diagnostic_bundle` — package OS logs/config/state for debugging.

/// Sentinel for `diagnostic_bundle`.
pub struct DiagnosticBundle;

cast::concept! {
    name: "diagnostic_bundle",
    summary: "package OS logs/config/state for debugging.",
    anchors: [cast_os_stdlib::observability::diagnostic_bundle::DiagnosticBundle],
    tags: ["cast_os_stdlib", "observability"],
}
