//! `container_image_layering` — immutable filesystem layers.

/// Sentinel for `container_image_layering`.
pub struct ContainerImageLayering;

cast::concept! {
    name: "container_image_layering",
    summary: "immutable filesystem layers.",
    anchors: [cast_os_stdlib::isolation::container_image_layering::ContainerImageLayering],
    tags: ["cast_os_stdlib", "isolation"],
}
