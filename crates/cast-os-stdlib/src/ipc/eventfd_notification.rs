//! `eventfd_notification` — Linux event counter notification.

/// Sentinel for `eventfd_notification`.
pub struct EventfdNotification;

cast::concept! {
    name: "eventfd_notification",
    summary: "Linux event counter notification.",
    anchors: [cast_os_stdlib::ipc::eventfd_notification::EventfdNotification],
    tags: ["cast_os_stdlib", "ipc"],
}
