//! `layered_architecture` — dependencies flow through layers.

/// Sentinel for `layered_architecture`.
pub struct LayeredArchitecture;

cast::concept! {
    name: "layered_architecture",
    summary: "Dependencies flow through layers. Higher layers depend on \
              lower; no circular dependencies, no skipping layers.",
    anchors: [cast_stdlib::architecture::layered_architecture::LayeredArchitecture],
    tags: ["cast_stdlib", "architecture"],
}
