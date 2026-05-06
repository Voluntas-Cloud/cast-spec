//! `monotonic_clock` — non-decreasing time source.

/// Sentinel for `monotonic_clock`.
pub struct MonotonicClock;

cast::concept! {
    name: "monotonic_clock",
    summary: "non-decreasing time source.",
    anchors: [cast_os_stdlib::interrupts_timers::monotonic_clock::MonotonicClock],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
