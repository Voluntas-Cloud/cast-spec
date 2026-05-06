//! `scheduler_tick` — periodic timer driving scheduling decisions.

/// Sentinel for `scheduler_tick`.
pub struct SchedulerTick;

cast::concept! {
    name: "scheduler_tick",
    summary: "periodic timer driving scheduling decisions.",
    anchors: [cast_os_stdlib::scheduling::scheduler_tick::SchedulerTick],
    tags: ["cast_os_stdlib", "scheduling"],
}
