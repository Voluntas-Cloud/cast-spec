//! `notification_service` — OS-level notification delivery.

/// Sentinel for `notification_service`.
pub struct NotificationService;

cast::concept! {
    name: "notification_service",
    summary: "OS-level notification delivery.",
    anchors: [cast_os_stdlib::desktop_session::notification_service::NotificationService],
    tags: ["cast_os_stdlib", "desktop_session"],
}
