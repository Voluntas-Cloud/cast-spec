//! `hardware_interrupt` — external device requests CPU attention.

/// Sentinel for `hardware_interrupt`.
pub struct HardwareInterrupt;

cast::concept! {
    name: "hardware_interrupt",
    summary: "external device requests CPU attention.",
    anchors: [cast_os_stdlib::interrupts_timers::hardware_interrupt::HardwareInterrupt],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
