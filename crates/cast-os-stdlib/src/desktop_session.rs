//! Desktop, session, and human interface OS concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod accessibility_tree;
pub mod audio_server_model;
pub mod clipboard_service;
pub mod credential_prompt_broker;
pub mod desktop_shell;
pub mod display_server;
pub mod drag_and_drop_protocol;
pub mod hotkey_dispatch_system;
pub mod input_method_framework;
pub mod notification_service;
pub mod portal_permission_model;
pub mod print_spooler;
pub mod screen_capture_permission_boundary;
pub mod session_lifecycle;
pub mod user_session_bus;
pub mod wayland_compositor_model;
pub mod windowing_system;
pub mod x11_networked_window_model;

cast::concept! {
    name: "desktop_session",
    summary: "Umbrella for the desktop_session stdlib category. Desktop, \
              session, and human interface OS concepts.",
    anchors: [
        crate::desktop_session::accessibility_tree,
        crate::desktop_session::audio_server_model,
        crate::desktop_session::clipboard_service,
        crate::desktop_session::credential_prompt_broker,
        crate::desktop_session::desktop_shell,
        crate::desktop_session::display_server,
        crate::desktop_session::drag_and_drop_protocol,
        crate::desktop_session::hotkey_dispatch_system,
        crate::desktop_session::input_method_framework,
        crate::desktop_session::notification_service,
        crate::desktop_session::portal_permission_model,
        crate::desktop_session::print_spooler,
        crate::desktop_session::screen_capture_permission_boundary,
        crate::desktop_session::session_lifecycle,
        crate::desktop_session::user_session_bus,
        crate::desktop_session::wayland_compositor_model,
        crate::desktop_session::windowing_system,
        crate::desktop_session::x11_networked_window_model,
    ],
    tags: ["cast_os_stdlib", "desktop_session"],
}

/// Sentinel for the desktop_session stdlib group.
pub struct DesktopSessionGroup;
