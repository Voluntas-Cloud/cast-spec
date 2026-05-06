//! `hotkey_dispatch_system` — global shortcut handling.

/// Sentinel for `hotkey_dispatch_system`.
pub struct HotkeyDispatchSystem;

cast::concept! {
    name: "hotkey_dispatch_system",
    summary: "global shortcut handling.",
    anchors: [cast_os_stdlib::desktop_session::hotkey_dispatch_system::HotkeyDispatchSystem],
    tags: ["cast_os_stdlib", "desktop_session"],
}
