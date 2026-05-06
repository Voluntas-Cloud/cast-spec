//! `build_cache` — reuse build outputs safely.

/// Sentinel for `build_cache`.
pub struct BuildCache;

cast::concept! {
    name: "build_cache",
    summary: "Reuse build outputs safely. Keyed on the inputs that \
              actually affect the output; correctness depends on \
              capturing every meaningful input in the cache key.",
    anchors: [cast_stdlib::supply_chain::build_cache::BuildCache],
    tags: ["cast_stdlib", "supply_chain"],
}
