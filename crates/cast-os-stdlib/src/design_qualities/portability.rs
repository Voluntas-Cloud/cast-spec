//! `portability` — can support multiple hardware platforms.

/// Sentinel for `portability`.
pub struct Portability;

cast::concept! {
    name: "portability",
    summary: "can support multiple hardware platforms.",
    anchors: [cast_os_stdlib::design_qualities::portability::Portability],
    tags: ["cast_os_stdlib", "design_qualities"],
}
