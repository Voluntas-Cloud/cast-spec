//! Architecture & boundaries patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod adapter_pattern;
pub mod anti_corruption_layer;
pub mod bounded_context;
pub mod classifier_dispatch;
pub mod control_plane_data_plane_split;
pub mod dependency_inversion;
pub mod event_driven_architecture;
pub mod facade_pattern;
pub mod hexagonal_architecture;
pub mod layered_architecture;
pub mod microkernel_architecture;
pub mod microservice_architecture;
pub mod modular_monolith;
pub mod module_boundary;
pub mod monolith_architecture;
pub mod plugin_architecture;
pub mod policy_engine_boundary;
pub mod service_boundary;
pub mod service_oriented_architecture;
pub mod sidecar_pattern;
pub mod strangler_fig_migration;
pub mod task_per_connection;
pub mod typed_handle;

cast::concept! {
    name: "architecture",
    summary: "Umbrella for the architecture stdlib category. Architecture \
              & boundaries patterns.",
    anchors: [
        crate::architecture::adapter_pattern,
        crate::architecture::anti_corruption_layer,
        crate::architecture::bounded_context,
        crate::architecture::classifier_dispatch,
        crate::architecture::control_plane_data_plane_split,
        crate::architecture::dependency_inversion,
        crate::architecture::event_driven_architecture,
        crate::architecture::facade_pattern,
        crate::architecture::hexagonal_architecture,
        crate::architecture::layered_architecture,
        crate::architecture::microkernel_architecture,
        crate::architecture::microservice_architecture,
        crate::architecture::modular_monolith,
        crate::architecture::module_boundary,
        crate::architecture::monolith_architecture,
        crate::architecture::plugin_architecture,
        crate::architecture::policy_engine_boundary,
        crate::architecture::service_boundary,
        crate::architecture::service_oriented_architecture,
        crate::architecture::sidecar_pattern,
        crate::architecture::strangler_fig_migration,
        crate::architecture::task_per_connection,
        crate::architecture::typed_handle,
    ],
    tags: ["cast_stdlib", "architecture"],
}

/// Sentinel for the architecture stdlib group.
pub struct ArchitectureGroup;

cast::rule! {
    rule: "Draw boundaries along ownership, change rate, and failure domains.",
    why:  "Org-chart cosplay produces boundaries that don't survive \
           the next reorg — and the code outlives the chart. \
           Boundaries that follow how the system actually changes \
           and fails survive the people who drew them.",
    governs: [cast_stdlib::architecture::ArchitectureGroup],
    tags: ["cast_stdlib", "architecture"],
}
