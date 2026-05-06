//! `facade_pattern` — simplified interface over complex subsystem.

/// Sentinel for `facade_pattern`.
pub struct FacadePattern;

cast::concept! {
    name: "facade_pattern",
    summary: "Simplified interface over complex subsystem. Hides \
              incidental complexity; presents the subset of capability \
              callers actually need.",
    anchors: [cast_stdlib::architecture::facade_pattern::FacadePattern],
    tags: ["cast_stdlib", "architecture"],
}
