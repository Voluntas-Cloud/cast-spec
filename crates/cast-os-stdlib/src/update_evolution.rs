//! Update, package, and system evolution.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod a_b_partition_update;
pub mod atomic_os_update;
pub mod declarative_system_configuration;
pub mod dependency_resolution;
pub mod driver_update_model;
pub mod firmware_update_model;
pub mod image_based_os_update;
pub mod immutable_os_base;
pub mod live_kernel_patching;
pub mod module_version_compatibility;
pub mod mutable_user_layer;
pub mod package_manager_model;
pub mod reproducible_system_build;
pub mod rollbackable_system_update;
pub mod signed_package_verification;
pub mod system_extension_model;
pub mod transactional_package_update;

cast::concept! {
    name: "update_evolution",
    summary: "Umbrella for the update_evolution stdlib category. Update, \
              package, and system evolution.",
    anchors: [
        crate::update_evolution::a_b_partition_update,
        crate::update_evolution::atomic_os_update,
        crate::update_evolution::declarative_system_configuration,
        crate::update_evolution::dependency_resolution,
        crate::update_evolution::driver_update_model,
        crate::update_evolution::firmware_update_model,
        crate::update_evolution::image_based_os_update,
        crate::update_evolution::immutable_os_base,
        crate::update_evolution::live_kernel_patching,
        crate::update_evolution::module_version_compatibility,
        crate::update_evolution::mutable_user_layer,
        crate::update_evolution::package_manager_model,
        crate::update_evolution::reproducible_system_build,
        crate::update_evolution::rollbackable_system_update,
        crate::update_evolution::signed_package_verification,
        crate::update_evolution::system_extension_model,
        crate::update_evolution::transactional_package_update,
    ],
    tags: ["cast_os_stdlib", "update_evolution"],
}

/// Sentinel for the update_evolution stdlib group.
pub struct UpdateEvolutionGroup;
