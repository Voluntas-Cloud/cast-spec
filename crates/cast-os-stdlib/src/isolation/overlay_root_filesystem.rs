//! `overlay_root_filesystem` — container writable layer over image.

/// Sentinel for `overlay_root_filesystem`.
pub struct OverlayRootFilesystem;

cast::concept! {
    name: "overlay_root_filesystem",
    summary: "container writable layer over image.",
    anchors: [cast_os_stdlib::isolation::overlay_root_filesystem::OverlayRootFilesystem],
    tags: ["cast_os_stdlib", "isolation"],
}
