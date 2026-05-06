//! `seccomp_container_boundary` — syscall filtering for containers.

/// Sentinel for `seccomp_container_boundary`.
pub struct SeccompContainerBoundary;

cast::concept! {
    name: "seccomp_container_boundary",
    summary: "syscall filtering for containers.",
    anchors: [cast_os_stdlib::isolation::seccomp_container_boundary::SeccompContainerBoundary],
    tags: ["cast_os_stdlib", "isolation"],
}
