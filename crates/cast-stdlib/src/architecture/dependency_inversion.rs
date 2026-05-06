//! `dependency_inversion` — high-level logic doesn't depend on concrete details.

/// Sentinel for `dependency_inversion`.
pub struct DependencyInversion;

cast::concept! {
    name: "dependency_inversion",
    summary: "High-level logic does not depend on concrete details. \
              Both depend on abstractions; the abstraction belongs to \
              the high-level layer.",
    anchors: [cast_stdlib::architecture::dependency_inversion::DependencyInversion],
    tags: ["cast_stdlib", "architecture"],
}
