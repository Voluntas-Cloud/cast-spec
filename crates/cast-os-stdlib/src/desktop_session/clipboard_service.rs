//! `clipboard_service` — cross-application clipboard.

/// Sentinel for `clipboard_service`.
pub struct ClipboardService;

cast::concept! {
    name: "clipboard_service",
    summary: "cross-application clipboard.",
    anchors: [cast_os_stdlib::desktop_session::clipboard_service::ClipboardService],
    tags: ["cast_os_stdlib", "desktop_session"],
}
