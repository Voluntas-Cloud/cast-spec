//! `container_escape` — isolated workload breaks out.

/// Sentinel for `container_escape`.
pub struct ContainerEscape;

cast::concept! {
    name: "container_escape",
    summary: "isolated workload breaks out.",
    anchors: [cast_os_stdlib::failure_modes::container_escape::ContainerEscape],
    tags: ["cast_os_stdlib", "failure_modes"],
}
