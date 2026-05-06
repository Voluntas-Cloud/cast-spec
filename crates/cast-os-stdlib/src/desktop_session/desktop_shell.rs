//! `desktop_shell` — user-facing desktop environment shell.

/// Sentinel for `desktop_shell`.
pub struct DesktopShell;

cast::concept! {
    name: "desktop_shell",
    summary: "user-facing desktop environment shell.",
    anchors: [cast_os_stdlib::desktop_session::desktop_shell::DesktopShell],
    tags: ["cast_os_stdlib", "desktop_session"],
}
