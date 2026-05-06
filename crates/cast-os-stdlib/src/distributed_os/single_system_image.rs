//! `single_system_image` — cluster appears as one OS.

/// Sentinel for `single_system_image`.
pub struct SingleSystemImage;

cast::concept! {
    name: "single_system_image",
    summary: "cluster appears as one OS.",
    anchors: [cast_os_stdlib::distributed_os::single_system_image::SingleSystemImage],
    tags: ["cast_os_stdlib", "distributed_os"],
}
