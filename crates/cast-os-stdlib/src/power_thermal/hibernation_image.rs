//! `hibernation_image` — saved RAM image for power-off restore.

/// Sentinel for `hibernation_image`.
pub struct HibernationImage;

cast::concept! {
    name: "hibernation_image",
    summary: "saved RAM image for power-off restore.",
    anchors: [cast_os_stdlib::power_thermal::hibernation_image::HibernationImage],
    tags: ["cast_os_stdlib", "power_thermal"],
}
