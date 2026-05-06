//! `interrupt_vector_table` — maps interrupts to handlers.

/// Sentinel for `interrupt_vector_table`.
pub struct InterruptVectorTable;

cast::concept! {
    name: "interrupt_vector_table",
    summary: "maps interrupts to handlers.",
    anchors: [cast_os_stdlib::interrupts_timers::interrupt_vector_table::InterruptVectorTable],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
