//! `timer_interrupt` — periodic or one-shot timer event.

/// Sentinel for `timer_interrupt`.
pub struct TimerInterrupt;

cast::concept! {
    name: "timer_interrupt",
    summary: "periodic or one-shot timer event.",
    anchors: [cast_os_stdlib::interrupts_timers::timer_interrupt::TimerInterrupt],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
