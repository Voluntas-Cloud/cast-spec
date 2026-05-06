//! State, cache & derived data patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod cache_invalidation;
pub mod cache_key_design;
pub mod cache_ttl;
pub mod cqrs_split;
pub mod derived_state;
pub mod eventual_projection;
pub mod materialized_view;
pub mod memoization;
pub mod precomputed_index;
pub mod read_model;
pub mod single_writer_principle;
pub mod source_of_truth_state;
pub mod stale_read_tolerance;
pub mod state_reconciliation;
pub mod write_model;

cast::concept! {
    name: "state_data",
    summary: "Umbrella for the state_data stdlib category. State, cache & \
              derived data patterns.",
    anchors: [
        crate::state_data::cache_invalidation,
        crate::state_data::cache_key_design,
        crate::state_data::cache_ttl,
        crate::state_data::cqrs_split,
        crate::state_data::derived_state,
        crate::state_data::eventual_projection,
        crate::state_data::materialized_view,
        crate::state_data::memoization,
        crate::state_data::precomputed_index,
        crate::state_data::read_model,
        crate::state_data::single_writer_principle,
        crate::state_data::source_of_truth_state,
        crate::state_data::stale_read_tolerance,
        crate::state_data::state_reconciliation,
        crate::state_data::write_model,
    ],
    tags: ["cast_stdlib", "state_data"],
}

/// Sentinel for the state_data stdlib group.
pub struct StateDataGroup;

cast::rule! {
    rule: "Every derived state needs a clear invalidation/rebuild story.",
    why:  "\"It updates somehow\" is not a design, it is a haunting. \
           The derived view will get out of sync; document how it \
           catches back up before that becomes the production \
           incident.",
    governs: [cast_stdlib::state_data::StateDataGroup],
    tags: ["cast_stdlib", "state_data"],
}
