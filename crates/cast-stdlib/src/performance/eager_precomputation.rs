//! `eager_precomputation` — compute ahead of time.

/// Sentinel for `eager_precomputation`.
pub struct EagerPrecomputation;

cast::concept! {
    name: "eager_precomputation",
    summary: "Compute ahead of time. Trades memory and storage for \
              latency at request time; pairs naturally with \
              materialized views and precomputed indexes.",
    anchors: [cast_stdlib::performance::eager_precomputation::EagerPrecomputation],
    tags: ["cast_stdlib", "performance"],
}
