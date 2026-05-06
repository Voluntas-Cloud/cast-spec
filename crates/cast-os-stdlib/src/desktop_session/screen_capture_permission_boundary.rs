//! `screen_capture_permission_boundary` — control screenshot/record access.

/// Sentinel for `screen_capture_permission_boundary`.
pub struct ScreenCapturePermissionBoundary;

cast::concept! {
    name: "screen_capture_permission_boundary",
    summary: "control screenshot/record access.",
    anchors: [cast_os_stdlib::desktop_session::screen_capture_permission_boundary::ScreenCapturePermissionBoundary],
    tags: ["cast_os_stdlib", "desktop_session"],
}
