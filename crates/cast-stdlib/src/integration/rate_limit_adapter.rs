//! `rate_limit_adapter` — respect external API limits.

/// Sentinel for `rate_limit_adapter`.
pub struct RateLimitAdapter;

cast::concept! {
    name: "rate_limit_adapter",
    summary: "Respect the external API's rate limits without \
              shifting the burden onto callers. Read the response \
              headers, back off proactively, and queue when needed; \
              don't make every consumer of the integration discover \
              429s independently.",
    anchors: [cast_stdlib::integration::rate_limit_adapter::RateLimitAdapter],
    tags: ["cast_stdlib", "integration"],
}
