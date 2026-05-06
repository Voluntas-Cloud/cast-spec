//! `scheduled_job` — time-based execution.

/// Sentinel for `scheduled_job`.
pub struct ScheduledJob;

cast::concept! {
    name: "scheduled_job",
    summary: "Time-based execution. The scheduler fires the job on a \
              calendar/interval; semantics for missed runs (skip, \
              coalesce, catch-up) must be explicit or operations \
              surprises follow.",
    anchors: [cast_stdlib::workflow::scheduled_job::ScheduledJob],
    tags: ["cast_stdlib", "workflow"],
}
