//! `x11_networked_window_model` — older network-transparent display model.

/// Sentinel for `x11_networked_window_model`.
pub struct X11NetworkedWindowModel;

cast::concept! {
    name: "x11_networked_window_model",
    summary: "older network-transparent display model.",
    anchors: [cast_os_stdlib::desktop_session::x11_networked_window_model::X11NetworkedWindowModel],
    tags: ["cast_os_stdlib", "desktop_session"],
}
