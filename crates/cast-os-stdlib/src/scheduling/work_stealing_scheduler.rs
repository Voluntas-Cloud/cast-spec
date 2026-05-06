//! `work_stealing_scheduler` — idle CPUs steal work from busy ones.

/// Sentinel for `work_stealing_scheduler`.
pub struct WorkStealingScheduler;

cast::concept! {
    name: "work_stealing_scheduler",
    summary: "idle CPUs steal work from busy ones.",
    anchors: [cast_os_stdlib::scheduling::work_stealing_scheduler::WorkStealingScheduler],
    tags: ["cast_os_stdlib", "scheduling"],
}
