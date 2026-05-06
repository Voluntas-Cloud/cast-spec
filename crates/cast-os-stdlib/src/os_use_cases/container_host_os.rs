//! `container_host_os` — minimal OS for container workloads.

/// Sentinel for `container_host_os`.
pub struct ContainerHostOs;

cast::concept! {
    name: "container_host_os",
    summary: "minimal OS for container workloads.",
    anchors: [cast_os_stdlib::os_use_cases::container_host_os::ContainerHostOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
