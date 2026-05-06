//! `cache_invalidation` — remove/update stale cached data.

/// Sentinel for `cache_invalidation`.
pub struct CacheInvalidation;

cast::concept! {
    name: "cache_invalidation",
    summary: "Remove or update cached data that has gone stale. The \
              hard part isn't writing to the cache; it's deciding \
              when the cached value no longer reflects reality. \
              Every cache needs a story for both reads and writes \
              that change the underlying data.",
    anchors: [cast_stdlib::state_data::cache_invalidation::CacheInvalidation],
    tags: ["cast_stdlib", "state_data"],
}
