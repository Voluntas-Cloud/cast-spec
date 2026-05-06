//! `rootless_container_model` — containers without host root privileges.

/// Sentinel for `rootless_container_model`.
pub struct RootlessContainerModel;

cast::concept! {
    name: "rootless_container_model",
    summary: "containers without host root privileges.",
    anchors: [cast_os_stdlib::isolation::rootless_container_model::RootlessContainerModel],
    tags: ["cast_os_stdlib", "isolation"],
}
