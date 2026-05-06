//! `anti_corruption_layer` — protect domain model from external model.

/// Sentinel for `anti_corruption_layer`.
pub struct AntiCorruptionLayer;

cast::concept! {
    name: "anti_corruption_layer",
    summary: "Protect domain model from external model. Translations \
              happen at the boundary; foreign concepts never leak into \
              the core.",
    anchors: [cast_stdlib::architecture::anti_corruption_layer::AntiCorruptionLayer],
    tags: ["cast_stdlib", "architecture"],
}
