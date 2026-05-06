//! `interrupt_coalescing` — batch interrupts to reduce overhead.

/// Sentinel for `interrupt_coalescing`.
pub struct InterruptCoalescing;

cast::concept! {
    name: "interrupt_coalescing",
    summary: "batch interrupts to reduce overhead.",
    anchors: [cast_os_stdlib::interrupts_timers::interrupt_coalescing::InterruptCoalescing],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
