//! `live_os_image` — bootable ephemeral OS image.

/// Sentinel for `live_os_image`.
pub struct LiveOsImage;

cast::concept! {
    name: "live_os_image",
    summary: "bootable ephemeral OS image.",
    anchors: [cast_os_stdlib::os_use_cases::live_os_image::LiveOsImage],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
