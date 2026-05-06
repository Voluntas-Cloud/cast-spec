//! `deterministic` — same inputs always produce the same output.

/// Sentinel for `deterministic`.
pub struct Deterministic;

cast::concept! {
    name: "deterministic",
    summary: "Identical inputs always produce identical output. \
              Weaker than pure_function — may have side effects \
              determined by the inputs (cursor advance, log line). \
              Replayable, cacheable.",
    anchors: [cast_stdlib::function_properties::deterministic::Deterministic],
    tags: ["cast_stdlib", "function_properties"],
}
