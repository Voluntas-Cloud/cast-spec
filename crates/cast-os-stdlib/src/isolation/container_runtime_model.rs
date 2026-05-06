//! `container_runtime_model` — create/manage isolated process environments.

/// Sentinel for `container_runtime_model`.
pub struct ContainerRuntimeModel;

cast::concept! {
    name: "container_runtime_model",
    summary: "create/manage isolated process environments.",
    anchors: [cast_os_stdlib::isolation::container_runtime_model::ContainerRuntimeModel],
    tags: ["cast_os_stdlib", "isolation"],
}
