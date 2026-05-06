//! `priority_inversion` — low-priority task blocks higher-priority task.

/// Sentinel for `priority_inversion`.
pub struct PriorityInversion;

cast::concept! {
    name: "priority_inversion",
    summary: "low-priority task blocks higher-priority task.",
    anchors: [cast_os_stdlib::scheduling::priority_inversion::PriorityInversion],
    tags: ["cast_os_stdlib", "scheduling"],
}
