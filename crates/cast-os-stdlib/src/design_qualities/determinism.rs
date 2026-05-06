//! `determinism` — predictable behavior.

/// Sentinel for `determinism`.
pub struct Determinism;

cast::concept! {
    name: "determinism",
    summary: "predictable behavior.",
    anchors: [cast_os_stdlib::design_qualities::determinism::Determinism],
    tags: ["cast_os_stdlib", "design_qualities"],
}
