//! `exception_trap` — CPU fault or exceptional condition.

/// Sentinel for `exception_trap`.
pub struct ExceptionTrap;

cast::concept! {
    name: "exception_trap",
    summary: "CPU fault or exceptional condition.",
    anchors: [cast_os_stdlib::interrupts_timers::exception_trap::ExceptionTrap],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
