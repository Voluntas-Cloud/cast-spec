//! `module_boundary` — explicit separation of code responsibility.

/// Sentinel for `module_boundary`.
pub struct ModuleBoundary;

cast::concept! {
    name: "module_boundary",
    summary: "Explicit separation of code responsibility. Modules expose \
              a deliberate surface; internal details are private.",
    anchors: [cast_stdlib::architecture::module_boundary::ModuleBoundary],
    tags: ["cast_stdlib", "architecture"],
}
