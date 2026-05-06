//! `event_triggered_job` — event-based execution.

/// Sentinel for `event_triggered_job`.
pub struct EventTriggeredJob;

cast::concept! {
    name: "event_triggered_job",
    summary: "Event-based execution. The job runs in response to a \
              fact (file landed, message arrived, state changed) \
              rather than a clock; idempotency and dedup decide whether \
              storms cause damage.",
    anchors: [cast_stdlib::workflow::event_triggered_job::EventTriggeredJob],
    tags: ["cast_stdlib", "workflow"],
}
