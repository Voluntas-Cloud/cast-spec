//! `cache_key_design` — deterministic identity for cached result.

/// Sentinel for `cache_key_design`.
pub struct CacheKeyDesign;

cast::concept! {
    name: "cache_key_design",
    summary: "Deterministic identity for the cached result. Forget \
              the tenant or the API version in the key, and the \
              cache happily serves one user's data to another or \
              freezes responses to a deprecated schema.",
    anchors: [cast_stdlib::state_data::cache_key_design::CacheKeyDesign],
    tags: ["cast_stdlib", "state_data"],
}
