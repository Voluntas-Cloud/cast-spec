//! `user_session_bus` — per-user IPC/control bus.

/// Sentinel for `user_session_bus`.
pub struct UserSessionBus;

cast::concept! {
    name: "user_session_bus",
    summary: "per-user IPC/control bus.",
    anchors: [cast_os_stdlib::desktop_session::user_session_bus::UserSessionBus],
    tags: ["cast_os_stdlib", "desktop_session"],
}
