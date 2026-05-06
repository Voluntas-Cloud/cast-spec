//! `rate_limiting` — restrict request rate.

/// Sentinel for `rate_limiting`.
pub struct RateLimiting;

cast::concept! {
    name: "rate_limiting",
    summary: "Restrict request rate. Token bucket, leaky bucket, \
              fixed window — bounds the load any single caller can \
              impose so one bad actor cannot drown the rest.",
    anchors: [cast_stdlib::performance::rate_limiting::RateLimiting],
    tags: ["cast_stdlib", "performance"],
}
