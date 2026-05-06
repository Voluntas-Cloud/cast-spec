//! `dependency_unit_graph` — service dependency graph.

/// Sentinel for `dependency_unit_graph`.
pub struct DependencyUnitGraph;

cast::concept! {
    name: "dependency_unit_graph",
    summary: "service dependency graph.",
    anchors: [cast_os_stdlib::service_management::dependency_unit_graph::DependencyUnitGraph],
    tags: ["cast_os_stdlib", "service_management"],
}
