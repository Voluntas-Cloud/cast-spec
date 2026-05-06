//! `negative_cache` — cache absence/failures.

/// Sentinel for `negative_cache`.
pub struct NegativeCache;

cast::concept! {
    name: "negative_cache",
    summary: "Cache absence/failures. 'Not found' or 'failed' are \
              answers worth caching too; prevents pathological retry \
              storms against a missing or broken backend.",
    anchors: [cast_stdlib::performance::negative_cache::NegativeCache],
    tags: ["cast_stdlib", "performance"],
}
