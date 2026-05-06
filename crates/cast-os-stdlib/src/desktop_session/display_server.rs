//! `display_server` — compositor/display manager service.

/// Sentinel for `display_server`.
pub struct DisplayServer;

cast::concept! {
    name: "display_server",
    summary: "compositor/display manager service.",
    anchors: [cast_os_stdlib::desktop_session::display_server::DisplayServer],
    tags: ["cast_os_stdlib", "desktop_session"],
}
