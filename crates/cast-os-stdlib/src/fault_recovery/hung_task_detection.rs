//! `hung_task_detection` — detect blocked tasks.

/// Sentinel for `hung_task_detection`.
pub struct HungTaskDetection;

cast::concept! {
    name: "hung_task_detection",
    summary: "detect blocked tasks.",
    anchors: [cast_os_stdlib::fault_recovery::hung_task_detection::HungTaskDetection],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
