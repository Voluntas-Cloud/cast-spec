//! `cache_projection_layer` — derived data cached or materialized for fast reads.

/// Sentinel for `cache_projection_layer`.
pub struct CacheProjectionLayer;

cast::concept! {
    name: "cache_projection_layer",
    summary: "Derived data is cached or materialised for fast reads. \
              Composes derived_state, materialized_view, \
              cache_key_design, cache_ttl, cache_invalidation, \
              eventual_projection, incremental_rebuild, and \
              stale_read_tolerance. Used for dashboards, search \
              indexes, AI retrieval indexes, reporting views, and UI \
              status aggregation.",
    anchors: [cast_stdlib::patterns::cache_projection_layer::CacheProjectionLayer],
    tags: ["cast_stdlib", "patterns"],
}
