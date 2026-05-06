//! `configfs_kernel_object_configuration` — create/configure kernel objects via filesystem.

/// Sentinel for `configfs_kernel_object_configuration`.
pub struct ConfigfsKernelObjectConfiguration;

cast::concept! {
    name: "configfs_kernel_object_configuration",
    summary: "create/configure kernel objects via filesystem.",
    anchors: [cast_os_stdlib::configuration::configfs_kernel_object_configuration::ConfigfsKernelObjectConfiguration],
    tags: ["cast_os_stdlib", "configuration"],
}
