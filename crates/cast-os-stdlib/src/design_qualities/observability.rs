//! `observability` — internal behavior can be inspected.

/// Sentinel for `observability`.
pub struct Observability;

cast::concept! {
    name: "observability",
    summary: "internal behavior can be inspected.",
    anchors: [cast_os_stdlib::design_qualities::observability::Observability],
    tags: ["cast_os_stdlib", "design_qualities"],
}
