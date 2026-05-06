//! `interrupt_controller` — hardware routing interrupts.

/// Sentinel for `interrupt_controller`.
pub struct InterruptController;

cast::concept! {
    name: "interrupt_controller",
    summary: "hardware routing interrupts.",
    anchors: [cast_os_stdlib::interrupts_timers::interrupt_controller::InterruptController],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
