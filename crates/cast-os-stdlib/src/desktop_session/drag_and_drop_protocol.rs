//! `drag_and_drop_protocol` — UI data transfer mechanism.

/// Sentinel for `drag_and_drop_protocol`.
pub struct DragAndDropProtocol;

cast::concept! {
    name: "drag_and_drop_protocol",
    summary: "UI data transfer mechanism.",
    anchors: [cast_os_stdlib::desktop_session::drag_and_drop_protocol::DragAndDropProtocol],
    tags: ["cast_os_stdlib", "desktop_session"],
}
