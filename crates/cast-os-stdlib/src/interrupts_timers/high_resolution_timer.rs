//! `high_resolution_timer` — precise timer event source.

/// Sentinel for `high_resolution_timer`.
pub struct HighResolutionTimer;

cast::concept! {
    name: "high_resolution_timer",
    summary: "precise timer event source.",
    anchors: [cast_os_stdlib::interrupts_timers::high_resolution_timer::HighResolutionTimer],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
