//! `watchdog_timer` — detects stuck system/component.

/// Sentinel for `watchdog_timer`.
pub struct WatchdogTimer;

cast::concept! {
    name: "watchdog_timer",
    summary: "detects stuck system/component.",
    anchors: [cast_os_stdlib::fault_recovery::watchdog_timer::WatchdogTimer],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
