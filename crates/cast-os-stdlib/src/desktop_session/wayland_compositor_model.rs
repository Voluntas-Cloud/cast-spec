//! `wayland_compositor_model` — compositor as display server and window manager.

/// Sentinel for `wayland_compositor_model`.
pub struct WaylandCompositorModel;

cast::concept! {
    name: "wayland_compositor_model",
    summary: "compositor as display server and window manager.",
    anchors: [cast_os_stdlib::desktop_session::wayland_compositor_model::WaylandCompositorModel],
    tags: ["cast_os_stdlib", "desktop_session"],
}
