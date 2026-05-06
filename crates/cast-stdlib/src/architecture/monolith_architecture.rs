//! `monolith_architecture` — one deployable unit, simpler operations.

/// Sentinel for `monolith_architecture`.
pub struct MonolithArchitecture;

cast::concept! {
    name: "monolith_architecture",
    summary: "One deployable unit. Simpler operations, harder to scale \
              teams against — every change touches the same artifact.",
    anchors: [cast_stdlib::architecture::monolith_architecture::MonolithArchitecture],
    tags: ["cast_stdlib", "architecture"],
}
