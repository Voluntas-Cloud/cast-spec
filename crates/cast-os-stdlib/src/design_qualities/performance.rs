//! `performance` — efficient resource use.

/// Sentinel for `performance`.
pub struct Performance;

cast::concept! {
    name: "performance",
    summary: "efficient resource use.",
    anchors: [cast_os_stdlib::design_qualities::performance::Performance],
    tags: ["cast_os_stdlib", "design_qualities"],
}
