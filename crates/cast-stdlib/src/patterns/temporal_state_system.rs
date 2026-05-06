//! `temporal_state_system` — answers "what was true at time or version X?"

/// Sentinel for `temporal_state_system`.
pub struct TemporalStateSystem;

cast::concept! {
    name: "temporal_state_system",
    summary: "A system can answer \"what was true at time or \
              version X?\" Composes mvcc_generation_log, \
              temporal_query, snapshot_storage, append_only_log, \
              logical_time, event_time, history_retention, and \
              versioned_identifier. Used for debugging historical \
              state, compliance review, undo/rollback systems, \
              configuration history, and financial/accounting \
              records.",
    anchors: [cast_stdlib::patterns::temporal_state_system::TemporalStateSystem],
    tags: ["cast_stdlib", "patterns"],
}
