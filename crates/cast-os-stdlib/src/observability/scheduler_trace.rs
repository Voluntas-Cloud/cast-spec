//! `scheduler_trace` — inspect scheduling behavior.

/// Sentinel for `scheduler_trace`.
pub struct SchedulerTrace;

cast::concept! {
    name: "scheduler_trace",
    summary: "inspect scheduling behavior.",
    anchors: [cast_os_stdlib::observability::scheduler_trace::SchedulerTrace],
    tags: ["cast_os_stdlib", "observability"],
}
