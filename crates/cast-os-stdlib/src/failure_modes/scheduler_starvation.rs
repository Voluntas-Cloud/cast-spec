//! `scheduler_starvation` — task never gets enough CPU.

/// Sentinel for `scheduler_starvation`.
pub struct SchedulerStarvation;

cast::concept! {
    name: "scheduler_starvation",
    summary: "task never gets enough CPU.",
    anchors: [cast_os_stdlib::failure_modes::scheduler_starvation::SchedulerStarvation],
    tags: ["cast_os_stdlib", "failure_modes"],
}
