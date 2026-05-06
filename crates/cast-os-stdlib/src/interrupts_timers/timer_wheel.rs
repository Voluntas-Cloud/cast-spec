//! `timer_wheel` — timer management data structure.

/// Sentinel for `timer_wheel`.
pub struct TimerWheel;

cast::concept! {
    name: "timer_wheel",
    summary: "timer management data structure.",
    anchors: [cast_os_stdlib::interrupts_timers::timer_wheel::TimerWheel],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
