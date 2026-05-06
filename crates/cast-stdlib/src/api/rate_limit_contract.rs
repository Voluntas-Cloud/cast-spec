//! `rate_limit_contract` — explicit allowed usage.

/// Sentinel for `rate_limit_contract`.
pub struct RateLimitContract;

cast::concept! {
    name: "rate_limit_contract",
    summary: "Explicit allowed usage. Callers know their budget; servers \
              respond with structured 429s when exceeded.",
    anchors: [cast_stdlib::api::rate_limit_contract::RateLimitContract],
    tags: ["cast_stdlib", "api"],
}
