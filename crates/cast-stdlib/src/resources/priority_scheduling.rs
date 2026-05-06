//! `priority_scheduling` — important work gets preference.

/// Sentinel for `priority_scheduling`.
pub struct PriorityScheduling;

cast::concept! {
    name: "priority_scheduling",
    summary: "Important work gets preference. Ordering is by declared \
              priority, not arrival; without preemption, low-priority \
              work in flight can still block until it completes.",
    anchors: [cast_stdlib::resources::priority_scheduling::PriorityScheduling],
    tags: ["cast_stdlib", "resources"],
}
