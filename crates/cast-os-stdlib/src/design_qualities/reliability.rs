//! `reliability` — continues working under faults.

/// Sentinel for `reliability`.
pub struct Reliability;

cast::concept! {
    name: "reliability",
    summary: "continues working under faults.",
    anchors: [cast_os_stdlib::design_qualities::reliability::Reliability],
    tags: ["cast_os_stdlib", "design_qualities"],
}
