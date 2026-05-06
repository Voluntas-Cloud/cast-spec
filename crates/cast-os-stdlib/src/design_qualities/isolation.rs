//! `isolation` — faults/authority do not leak.

/// Sentinel for `isolation`.
pub struct Isolation;

cast::concept! {
    name: "isolation",
    summary: "faults/authority do not leak.",
    anchors: [cast_os_stdlib::design_qualities::isolation::Isolation],
    tags: ["cast_os_stdlib", "design_qualities"],
}
