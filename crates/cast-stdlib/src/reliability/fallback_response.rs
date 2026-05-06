//! `fallback_response` — degraded alternative when primary fails.

/// Sentinel for `fallback_response`.
pub struct FallbackResponse;

cast::concept! {
    name: "fallback_response",
    summary: "Use degraded alternative when primary fails. Cached value, \
              default, or simpler computation — better than an error \
              when partial answers are acceptable.",
    anchors: [cast_stdlib::reliability::fallback_response::FallbackResponse],
    tags: ["cast_stdlib", "reliability"],
}
