//! `capability_oriented_os_model` — authority modeled as possession of capabilities.

/// Sentinel for `capability_oriented_os_model`.
pub struct CapabilityOrientedOsModel;

cast::concept! {
    name: "capability_oriented_os_model",
    summary: "authority modeled as possession of capabilities.",
    anchors: [cast_os_stdlib::architectural_patterns::capability_oriented_os_model::CapabilityOrientedOsModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
