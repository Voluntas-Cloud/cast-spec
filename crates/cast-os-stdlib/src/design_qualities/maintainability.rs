//! `maintainability` — can evolve safely.

/// Sentinel for `maintainability`.
pub struct Maintainability;

cast::concept! {
    name: "maintainability",
    summary: "can evolve safely.",
    anchors: [cast_os_stdlib::design_qualities::maintainability::Maintainability],
    tags: ["cast_os_stdlib", "design_qualities"],
}
