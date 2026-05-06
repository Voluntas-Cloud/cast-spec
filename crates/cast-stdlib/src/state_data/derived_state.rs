//! `derived_state` — computed from source.

/// Sentinel for `derived_state`.
pub struct DerivedState;

cast::concept! {
    name: "derived_state",
    summary: "State computed from the source of truth — projections, \
              aggregates, summaries. Treat them as caches with a \
              rebuild story; if the rebuild is impossible, the \
              \"derived\" data is in fact a second source of truth in \
              disguise.",
    anchors: [cast_stdlib::state_data::derived_state::DerivedState],
    tags: ["cast_stdlib", "state_data"],
}
