//! OS design qualities.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod availability;
pub mod compatibility;
pub mod debuggability;
pub mod determinism;
pub mod energy_efficiency;
pub mod extensibility;
pub mod fault_containment;
pub mod isolation;
pub mod latency_predictability;
pub mod maintainability;
pub mod mechanism_generality;
pub mod minimality;
pub mod observability;
pub mod performance;
pub mod policy_flexibility;
pub mod portability;
pub mod reliability;
pub mod scalability;
pub mod security;
pub mod usability;

cast::concept! {
    name: "design_qualities",
    summary: "Umbrella for the design_qualities stdlib category. OS design \
              qualities.",
    anchors: [
        crate::design_qualities::availability,
        crate::design_qualities::compatibility,
        crate::design_qualities::debuggability,
        crate::design_qualities::determinism,
        crate::design_qualities::energy_efficiency,
        crate::design_qualities::extensibility,
        crate::design_qualities::fault_containment,
        crate::design_qualities::isolation,
        crate::design_qualities::latency_predictability,
        crate::design_qualities::maintainability,
        crate::design_qualities::mechanism_generality,
        crate::design_qualities::minimality,
        crate::design_qualities::observability,
        crate::design_qualities::performance,
        crate::design_qualities::policy_flexibility,
        crate::design_qualities::portability,
        crate::design_qualities::reliability,
        crate::design_qualities::scalability,
        crate::design_qualities::security,
        crate::design_qualities::usability,
    ],
    tags: ["cast_os_stdlib", "design_qualities"],
}

/// Sentinel for the design_qualities stdlib group.
pub struct DesignQualitiesGroup;
