//! `cache_ttl` — expire cache after time.

/// Sentinel for `cache_ttl`.
pub struct CacheTtl;

cast::concept! {
    name: "cache_ttl",
    summary: "Expire cache entries after a fixed time. The crudest \
              invalidation strategy and often the right one — bounded \
              staleness without coupling the cache to every writer. \
              The TTL value itself is a design choice, not a default.",
    anchors: [cast_stdlib::state_data::cache_ttl::CacheTtl],
    tags: ["cast_stdlib", "state_data"],
}
