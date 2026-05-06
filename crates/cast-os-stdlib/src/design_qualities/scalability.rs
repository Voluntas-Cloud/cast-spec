//! `scalability` — works from tiny to large systems.

/// Sentinel for `scalability`.
pub struct Scalability;

cast::concept! {
    name: "scalability",
    summary: "works from tiny to large systems.",
    anchors: [cast_os_stdlib::design_qualities::scalability::Scalability],
    tags: ["cast_os_stdlib", "design_qualities"],
}
