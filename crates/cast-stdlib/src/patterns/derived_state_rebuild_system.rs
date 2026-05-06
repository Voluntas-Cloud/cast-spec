//! `derived_state_rebuild_system` — derived indexes/projections rebuildable from sources.

/// Sentinel for `derived_state_rebuild_system`.
pub struct DerivedStateRebuildSystem;

cast::concept! {
    name: "derived_state_rebuild_system",
    summary: "Derived indexes, projections, caches, and summaries \
              can be rebuilt from sources. Composes \
              source_of_truth_state, derived_state, \
              incremental_rebuild, content_hash_id, event_stream, \
              materialized_view, cache_invalidation, and \
              checkpoint_resume. Used for search index rebuilds, AI \
              embedding refresh, dashboard recomputation, cache \
              recovery, and projection repair.",
    anchors: [cast_stdlib::patterns::derived_state_rebuild_system::DerivedStateRebuildSystem],
    tags: ["cast_stdlib", "patterns"],
}
