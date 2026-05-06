//! `processing_time` — when system processed event.

/// Sentinel for `processing_time`.
pub struct ProcessingTime;

cast::concept! {
    name: "processing_time",
    summary: "When the system actually processed the event. Cheap to \
              produce and reason about; useful for SLOs and dashboards \
              about the pipeline itself, but a poor proxy for event \
              time when there's any backlog at all.",
    anchors: [cast_stdlib::time_ordering::processing_time::ProcessingTime],
    tags: ["cast_stdlib", "time_ordering"],
}
