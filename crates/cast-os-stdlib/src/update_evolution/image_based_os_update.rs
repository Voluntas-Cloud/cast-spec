//! `image_based_os_update` — update whole OS image.

/// Sentinel for `image_based_os_update`.
pub struct ImageBasedOsUpdate;

cast::concept! {
    name: "image_based_os_update",
    summary: "update whole OS image.",
    anchors: [cast_os_stdlib::update_evolution::image_based_os_update::ImageBasedOsUpdate],
    tags: ["cast_os_stdlib", "update_evolution"],
}
