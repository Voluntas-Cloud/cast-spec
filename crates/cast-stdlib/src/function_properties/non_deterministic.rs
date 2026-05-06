//! `non_deterministic` — same inputs may produce different outputs across calls.

/// Sentinel for `non_deterministic`.
pub struct NonDeterministic;

cast::concept! {
    name: "non_deterministic",
    summary: "Same inputs may produce different outputs across calls. \
              Source: clock, RNG, network, filesystem, scheduling. \
              Required for randomness, time, concurrency — not a \
              defect, a contract.",
    anchors: [cast_stdlib::function_properties::non_deterministic::NonDeterministic],
    tags: ["cast_stdlib", "function_properties"],
}
