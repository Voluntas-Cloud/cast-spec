//! `windowing_system` — manages graphical windows.

/// Sentinel for `windowing_system`.
pub struct WindowingSystem;

cast::concept! {
    name: "windowing_system",
    summary: "manages graphical windows.",
    anchors: [cast_os_stdlib::desktop_session::windowing_system::WindowingSystem],
    tags: ["cast_os_stdlib", "desktop_session"],
}
