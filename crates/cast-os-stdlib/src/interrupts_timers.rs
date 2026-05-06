//! Interrupts, timers, and CPU events.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod advanced_programmable_interrupt_controller;
pub mod clock_event_device;
pub mod clocksource_selection;
pub mod deferred_procedure_call;
pub mod exception_trap;
pub mod hardware_interrupt;
pub mod high_resolution_timer;
pub mod hrtimer_tree;
pub mod interrupt_coalescing;
pub mod interrupt_controller;
pub mod interrupt_masking;
pub mod interrupt_vector_table;
pub mod monotonic_clock;
pub mod msi_interrupts;
pub mod software_interrupt;
pub mod timekeeping_subsystem;
pub mod timer_interrupt;
pub mod timer_wheel;
pub mod wall_clock_source;

cast::concept! {
    name: "interrupts_timers",
    summary: "Umbrella for the interrupts_timers stdlib category. \
              Interrupts, timers, and CPU events.",
    anchors: [
        crate::interrupts_timers::advanced_programmable_interrupt_controller,
        crate::interrupts_timers::clock_event_device,
        crate::interrupts_timers::clocksource_selection,
        crate::interrupts_timers::deferred_procedure_call,
        crate::interrupts_timers::exception_trap,
        crate::interrupts_timers::hardware_interrupt,
        crate::interrupts_timers::high_resolution_timer,
        crate::interrupts_timers::hrtimer_tree,
        crate::interrupts_timers::interrupt_coalescing,
        crate::interrupts_timers::interrupt_controller,
        crate::interrupts_timers::interrupt_masking,
        crate::interrupts_timers::interrupt_vector_table,
        crate::interrupts_timers::monotonic_clock,
        crate::interrupts_timers::msi_interrupts,
        crate::interrupts_timers::software_interrupt,
        crate::interrupts_timers::timekeeping_subsystem,
        crate::interrupts_timers::timer_interrupt,
        crate::interrupts_timers::timer_wheel,
        crate::interrupts_timers::wall_clock_source,
    ],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}

/// Sentinel for the interrupts_timers stdlib group.
pub struct InterruptsTimersGroup;
