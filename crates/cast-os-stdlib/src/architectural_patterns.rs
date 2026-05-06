//! OS architectural patterns.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod adaptive_control_loop_os;
pub mod brokered_access_model;
pub mod capability_oriented_os_model;
pub mod declarative_os_configuration_model;
pub mod event_driven_kernel_model;
pub mod everything_is_a_file_model;
pub mod everything_is_an_object_model;
pub mod formally_verified_kernel_core;
pub mod handle_based_authority_model;
pub mod hybrid_interrupt_polling_model;
pub mod immutable_base_mutable_overlay_os;
pub mod kernel_fast_path_user_policy_model;
pub mod least_privilege_service_os;
pub mod namespace_based_resource_model;
pub mod policy_driven_os_control_plane;
pub mod pull_based_device_polling_model;
pub mod push_based_interrupt_model;
pub mod self_describing_os_model;
pub mod transactional_os_state_model;
pub mod user_space_service_os;

cast::concept! {
    name: "architectural_patterns",
    summary: "Umbrella for the architectural_patterns stdlib category. OS \
              architectural patterns.",
    anchors: [
        crate::architectural_patterns::adaptive_control_loop_os,
        crate::architectural_patterns::brokered_access_model,
        crate::architectural_patterns::capability_oriented_os_model,
        crate::architectural_patterns::declarative_os_configuration_model,
        crate::architectural_patterns::event_driven_kernel_model,
        crate::architectural_patterns::everything_is_a_file_model,
        crate::architectural_patterns::everything_is_an_object_model,
        crate::architectural_patterns::formally_verified_kernel_core,
        crate::architectural_patterns::handle_based_authority_model,
        crate::architectural_patterns::hybrid_interrupt_polling_model,
        crate::architectural_patterns::immutable_base_mutable_overlay_os,
        crate::architectural_patterns::kernel_fast_path_user_policy_model,
        crate::architectural_patterns::least_privilege_service_os,
        crate::architectural_patterns::namespace_based_resource_model,
        crate::architectural_patterns::policy_driven_os_control_plane,
        crate::architectural_patterns::pull_based_device_polling_model,
        crate::architectural_patterns::push_based_interrupt_model,
        crate::architectural_patterns::self_describing_os_model,
        crate::architectural_patterns::transactional_os_state_model,
        crate::architectural_patterns::user_space_service_os,
    ],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}

/// Sentinel for the architectural_patterns stdlib group.
pub struct ArchitecturalPatternsGroup;
