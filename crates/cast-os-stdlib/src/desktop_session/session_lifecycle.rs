//! `session_lifecycle` — login, lock, unlock, logout.

/// Sentinel for `session_lifecycle`.
pub struct SessionLifecycle;

cast::concept! {
    name: "session_lifecycle",
    summary: "login, lock, unlock, logout.",
    anchors: [cast_os_stdlib::desktop_session::session_lifecycle::SessionLifecycle],
    tags: ["cast_os_stdlib", "desktop_session"],
}
