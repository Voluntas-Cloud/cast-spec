//! `circuit_breaker` — stop calling a failing dependency temporarily.

/// Sentinel for `circuit_breaker`.
pub struct CircuitBreaker;

cast::concept! {
    name: "circuit_breaker",
    summary: "Stop calling failing dependency temporarily. After enough \
              consecutive failures the breaker opens; calls fast-fail \
              until a probe succeeds and the breaker closes again.",
    anchors: [cast_stdlib::reliability::circuit_breaker::CircuitBreaker],
    tags: ["cast_stdlib", "reliability"],
}
