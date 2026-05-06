//! `timekeeping_subsystem` — OS clock management.

/// Sentinel for `timekeeping_subsystem`.
pub struct TimekeepingSubsystem;

cast::concept! {
    name: "timekeeping_subsystem",
    summary: "OS clock management.",
    anchors: [cast_os_stdlib::interrupts_timers::timekeeping_subsystem::TimekeepingSubsystem],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
