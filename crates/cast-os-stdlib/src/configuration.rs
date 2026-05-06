//! Configuration and registry concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod boot_time_configuration;
pub mod configfs_kernel_object_configuration;
pub mod configuration_drift_detection;
pub mod configuration_layering;
pub mod configuration_rollback;
pub mod configuration_transaction;
pub mod environment_variable_configuration;
pub mod hierarchical_configuration_database;
pub mod plain_file_configuration;
pub mod policy_configuration_split;
pub mod procfs_configuration_view;
pub mod runtime_mutable_configuration;
pub mod sysctl_runtime_configuration;
pub mod sysfs_device_model_view;
pub mod windows_registry_model;

cast::concept! {
    name: "configuration",
    summary: "Umbrella for the configuration stdlib category. \
              Configuration and registry concepts.",
    anchors: [
        crate::configuration::boot_time_configuration,
        crate::configuration::configfs_kernel_object_configuration,
        crate::configuration::configuration_drift_detection,
        crate::configuration::configuration_layering,
        crate::configuration::configuration_rollback,
        crate::configuration::configuration_transaction,
        crate::configuration::environment_variable_configuration,
        crate::configuration::hierarchical_configuration_database,
        crate::configuration::plain_file_configuration,
        crate::configuration::policy_configuration_split,
        crate::configuration::procfs_configuration_view,
        crate::configuration::runtime_mutable_configuration,
        crate::configuration::sysctl_runtime_configuration,
        crate::configuration::sysfs_device_model_view,
        crate::configuration::windows_registry_model,
    ],
    tags: ["cast_os_stdlib", "configuration"],
}

/// Sentinel for the configuration stdlib group.
pub struct ConfigurationGroup;
