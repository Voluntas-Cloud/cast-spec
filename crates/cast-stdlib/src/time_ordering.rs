//! Time, ordering & history patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod audit_history;
pub mod causal_ordering;
pub mod clock_skew_budget;
pub mod event_time;
pub mod history_retention;
pub mod ingestion_time;
pub mod late_event_handling;
pub mod logical_time;
pub mod monotonic_time;
pub mod processing_time;
pub mod sequence_numbering;
pub mod temporal_query;
pub mod time_windowing;
pub mod wall_clock_time;
pub mod watermark;

cast::concept! {
    name: "time_ordering",
    summary: "Umbrella for the time_ordering stdlib category. Time, \
              ordering & history patterns.",
    anchors: [
        crate::time_ordering::audit_history,
        crate::time_ordering::causal_ordering,
        crate::time_ordering::clock_skew_budget,
        crate::time_ordering::event_time,
        crate::time_ordering::history_retention,
        crate::time_ordering::ingestion_time,
        crate::time_ordering::late_event_handling,
        crate::time_ordering::logical_time,
        crate::time_ordering::monotonic_time,
        crate::time_ordering::processing_time,
        crate::time_ordering::sequence_numbering,
        crate::time_ordering::temporal_query,
        crate::time_ordering::time_windowing,
        crate::time_ordering::wall_clock_time,
        crate::time_ordering::watermark,
    ],
    tags: ["cast_stdlib", "time_ordering"],
}

/// Sentinel for the time_ordering stdlib group.
pub struct TimeOrderingGroup;

cast::rule! {
    rule: "Distinguish \"when it happened\" from \"when we saw it\".",
    why:  "The universe was inconsiderate enough to make those \
           different. Conflating them is how out-of-order events \
           silently corrupt your aggregations and how late data \
           becomes \"we're missing yesterday\".",
    governs: [cast_stdlib::time_ordering::TimeOrderingGroup],
    tags: ["cast_stdlib", "time_ordering"],
}
