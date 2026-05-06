//! `event_time` — when event actually occurred.

/// Sentinel for `event_time`.
pub struct EventTime;

cast::concept! {
    name: "event_time",
    summary: "When the event actually happened in the world. The \
              source of truth for analytics and aggregations; \
              pipelines that group by processing time give you \
              answers, just not the right ones.",
    anchors: [cast_stdlib::time_ordering::event_time::EventTime],
    tags: ["cast_stdlib", "time_ordering"],
}
