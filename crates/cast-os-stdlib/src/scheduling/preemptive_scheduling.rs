//! `preemptive_scheduling` — scheduler can interrupt running tasks.

/// Sentinel for `preemptive_scheduling`.
pub struct PreemptiveScheduling;

cast::concept! {
    name: "preemptive_scheduling",
    summary: "scheduler can interrupt running tasks.",
    anchors: [cast_os_stdlib::scheduling::preemptive_scheduling::PreemptiveScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
