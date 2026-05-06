//! Service management and user-space OS control plane.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod configuration_drop_in;
pub mod dependency_unit_graph;
pub mod least_privilege_service;
pub mod logging_journal;
pub mod one_shot_service;
pub mod path_activation;
pub mod restart_policy;
pub mod service_account_identity;
pub mod service_readiness_protocol;
pub mod service_sandboxing;
pub mod service_unit;
pub mod session_manager;
pub mod socket_activation;
pub mod target_unit;
pub mod timer_activation;
pub mod user_service_manager;
pub mod watchdog_notification;

cast::concept! {
    name: "service_management",
    summary: "Umbrella for the service_management stdlib category. Service \
              management and user-space OS control plane.",
    anchors: [
        crate::service_management::configuration_drop_in,
        crate::service_management::dependency_unit_graph,
        crate::service_management::least_privilege_service,
        crate::service_management::logging_journal,
        crate::service_management::one_shot_service,
        crate::service_management::path_activation,
        crate::service_management::restart_policy,
        crate::service_management::service_account_identity,
        crate::service_management::service_readiness_protocol,
        crate::service_management::service_sandboxing,
        crate::service_management::service_unit,
        crate::service_management::session_manager,
        crate::service_management::socket_activation,
        crate::service_management::target_unit,
        crate::service_management::timer_activation,
        crate::service_management::user_service_manager,
        crate::service_management::watchdog_notification,
    ],
    tags: ["cast_os_stdlib", "service_management"],
}

/// Sentinel for the service_management stdlib group.
pub struct ServiceManagementGroup;
