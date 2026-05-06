//! `portal_permission_model` — desktop-mediated app permissions.

/// Sentinel for `portal_permission_model`.
pub struct PortalPermissionModel;

cast::concept! {
    name: "portal_permission_model",
    summary: "desktop-mediated app permissions.",
    anchors: [cast_os_stdlib::desktop_session::portal_permission_model::PortalPermissionModel],
    tags: ["cast_os_stdlib", "desktop_session"],
}
