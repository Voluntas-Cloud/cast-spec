//! `precomputed_index` — maintain lookup structure.

/// Sentinel for `precomputed_index`.
pub struct PrecomputedIndex;

cast::concept! {
    name: "precomputed_index",
    summary: "A lookup structure built ahead of read time. Trades \
              write cost for read latency; the index is itself \
              derived state and needs the same invalidation story \
              everything else derived needs.",
    anchors: [cast_stdlib::state_data::precomputed_index::PrecomputedIndex],
    tags: ["cast_stdlib", "state_data"],
}
