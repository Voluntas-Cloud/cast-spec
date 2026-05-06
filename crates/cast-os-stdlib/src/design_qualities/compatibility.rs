//! `compatibility` — old software keeps working.

/// Sentinel for `compatibility`.
pub struct Compatibility;

cast::concept! {
    name: "compatibility",
    summary: "old software keeps working.",
    anchors: [cast_os_stdlib::design_qualities::compatibility::Compatibility],
    tags: ["cast_os_stdlib", "design_qualities"],
}
