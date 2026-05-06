//! `software_interrupt` — software-triggered interrupt/trap.

/// Sentinel for `software_interrupt`.
pub struct SoftwareInterrupt;

cast::concept! {
    name: "software_interrupt",
    summary: "software-triggered interrupt/trap.",
    anchors: [cast_os_stdlib::interrupts_timers::software_interrupt::SoftwareInterrupt],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
