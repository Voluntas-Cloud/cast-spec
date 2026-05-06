//! Isolation, containers, and namespaces.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod cgroup_resource_control;
pub mod chroot_filesystem_jail;
pub mod container_escape_risk;
pub mod container_image_layering;
pub mod container_runtime_model;
pub mod ipc_namespace;
pub mod jail_isolation_model;
pub mod mount_namespace;
pub mod namespace_isolation;
pub mod network_namespace;
pub mod overlay_root_filesystem;
pub mod pid_namespace;
pub mod pod_sandbox_model;
pub mod process_isolation;
pub mod rootless_container_model;
pub mod seccomp_container_boundary;
pub mod sidecar_process_pattern;
pub mod user_namespace;
pub mod uts_namespace;
pub mod zone_isolation_model;

cast::concept! {
    name: "isolation",
    summary: "Umbrella for the isolation stdlib category. Isolation, \
              containers, and namespaces.",
    anchors: [
        crate::isolation::cgroup_resource_control,
        crate::isolation::chroot_filesystem_jail,
        crate::isolation::container_escape_risk,
        crate::isolation::container_image_layering,
        crate::isolation::container_runtime_model,
        crate::isolation::ipc_namespace,
        crate::isolation::jail_isolation_model,
        crate::isolation::mount_namespace,
        crate::isolation::namespace_isolation,
        crate::isolation::network_namespace,
        crate::isolation::overlay_root_filesystem,
        crate::isolation::pid_namespace,
        crate::isolation::pod_sandbox_model,
        crate::isolation::process_isolation,
        crate::isolation::rootless_container_model,
        crate::isolation::seccomp_container_boundary,
        crate::isolation::sidecar_process_pattern,
        crate::isolation::user_namespace,
        crate::isolation::uts_namespace,
        crate::isolation::zone_isolation_model,
    ],
    tags: ["cast_os_stdlib", "isolation"],
}

/// Sentinel for the isolation stdlib group.
pub struct IsolationGroup;
