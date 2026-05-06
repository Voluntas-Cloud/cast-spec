//! `overlay_filesystem` — writable layer over read-only lower layers.

/// Sentinel for `overlay_filesystem`.
pub struct OverlayFilesystem;

cast::concept! {
    name: "overlay_filesystem",
    summary: "writable layer over read-only lower layers.",
    anchors: [cast_os_stdlib::filesystem_storage::overlay_filesystem::OverlayFilesystem],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
