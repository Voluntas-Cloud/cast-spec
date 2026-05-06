//! `debuggability` — failures can be understood.

/// Sentinel for `debuggability`.
pub struct Debuggability;

cast::concept! {
    name: "debuggability",
    summary: "failures can be understood.",
    anchors: [cast_os_stdlib::design_qualities::debuggability::Debuggability],
    tags: ["cast_os_stdlib", "design_qualities"],
}
