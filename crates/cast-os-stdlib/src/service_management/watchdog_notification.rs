//! `watchdog_notification` — service reports health to supervisor.

/// Sentinel for `watchdog_notification`.
pub struct WatchdogNotification;

cast::concept! {
    name: "watchdog_notification",
    summary: "service reports health to supervisor.",
    anchors: [cast_os_stdlib::service_management::watchdog_notification::WatchdogNotification],
    tags: ["cast_os_stdlib", "service_management"],
}
