//! `wall_clock_source` — real-world time source.

/// Sentinel for `wall_clock_source`.
pub struct WallClockSource;

cast::concept! {
    name: "wall_clock_source",
    summary: "real-world time source.",
    anchors: [cast_os_stdlib::interrupts_timers::wall_clock_source::WallClockSource],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
