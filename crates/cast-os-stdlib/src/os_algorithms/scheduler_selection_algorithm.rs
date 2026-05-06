//! `scheduler_selection_algorithm` — choose next task.

/// Sentinel for `scheduler_selection_algorithm`.
pub struct SchedulerSelectionAlgorithm;

cast::concept! {
    name: "scheduler_selection_algorithm",
    summary: "choose next task.",
    anchors: [cast_os_stdlib::os_algorithms::scheduler_selection_algorithm::SchedulerSelectionAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
