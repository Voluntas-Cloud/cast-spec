//! `session_manager` — user login/session lifecycle manager.

/// Sentinel for `session_manager`.
pub struct SessionManager;

cast::concept! {
    name: "session_manager",
    summary: "user login/session lifecycle manager.",
    anchors: [cast_os_stdlib::service_management::session_manager::SessionManager],
    tags: ["cast_os_stdlib", "service_management"],
}
