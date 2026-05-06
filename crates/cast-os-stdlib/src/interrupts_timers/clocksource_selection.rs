//! `clocksource_selection` — choose hardware clock source.

/// Sentinel for `clocksource_selection`.
pub struct ClocksourceSelection;

cast::concept! {
    name: "clocksource_selection",
    summary: "choose hardware clock source.",
    anchors: [cast_os_stdlib::interrupts_timers::clocksource_selection::ClocksourceSelection],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
