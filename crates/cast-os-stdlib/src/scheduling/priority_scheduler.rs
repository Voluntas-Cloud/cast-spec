//! `priority_scheduler` — tasks selected by priority.

/// Sentinel for `priority_scheduler`.
pub struct PriorityScheduler;

cast::concept! {
    name: "priority_scheduler",
    summary: "tasks selected by priority.",
    anchors: [cast_os_stdlib::scheduling::priority_scheduler::PriorityScheduler],
    tags: ["cast_os_stdlib", "scheduling"],
}
