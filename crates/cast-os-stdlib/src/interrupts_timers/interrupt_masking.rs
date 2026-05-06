//! `interrupt_masking` — temporarily disable selected interrupts.

/// Sentinel for `interrupt_masking`.
pub struct InterruptMasking;

cast::concept! {
    name: "interrupt_masking",
    summary: "temporarily disable selected interrupts.",
    anchors: [cast_os_stdlib::interrupts_timers::interrupt_masking::InterruptMasking],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
