//! `advanced_programmable_interrupt_controller` — APIC-style interrupt control.

/// Sentinel for `advanced_programmable_interrupt_controller`.
pub struct AdvancedProgrammableInterruptController;

cast::concept! {
    name: "advanced_programmable_interrupt_controller",
    summary: "APIC-style interrupt control.",
    anchors: [cast_os_stdlib::interrupts_timers::advanced_programmable_interrupt_controller::AdvancedProgrammableInterruptController],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
