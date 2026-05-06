//! `clock_event_device` — device that schedules timer events.

/// Sentinel for `clock_event_device`.
pub struct ClockEventDevice;

cast::concept! {
    name: "clock_event_device",
    summary: "device that schedules timer events.",
    anchors: [cast_os_stdlib::interrupts_timers::clock_event_device::ClockEventDevice],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
