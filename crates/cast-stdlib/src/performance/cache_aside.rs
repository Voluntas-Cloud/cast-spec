//! `cache_aside` — application manages cache lookup/fill.

/// Sentinel for `cache_aside`.
pub struct CacheAside;

cast::concept! {
    name: "cache_aside",
    summary: "Application manages cache lookup/fill. Reader checks \
              cache first; on miss, fetches from source and populates \
              the cache. Writes go straight to the source.",
    anchors: [cast_stdlib::performance::cache_aside::CacheAside],
    tags: ["cast_stdlib", "performance"],
}
