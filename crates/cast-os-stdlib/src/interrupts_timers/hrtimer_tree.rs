//! `hrtimer_tree` — high-resolution timer ordering structure.

/// Sentinel for `hrtimer_tree`.
pub struct HrtimerTree;

cast::concept! {
    name: "hrtimer_tree",
    summary: "high-resolution timer ordering structure.",
    anchors: [cast_os_stdlib::interrupts_timers::hrtimer_tree::HrtimerTree],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
