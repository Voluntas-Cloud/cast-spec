//! `timer_wheel` — efficient timer bucket structure.

/// Sentinel for `timer_wheel`.
pub struct TimerWheel;

cast::concept! {
    name: "timer_wheel",
    summary: "efficient timer bucket structure.",
    anchors: [cast_os_stdlib::kernel_data_structures::timer_wheel::TimerWheel],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
