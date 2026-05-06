//! `cpu_topology_model` — cores, sockets, SMT relationships.

/// Sentinel for `cpu_topology_model`.
pub struct CpuTopologyModel;

cast::concept! {
    name: "cpu_topology_model",
    summary: "cores, sockets, SMT relationships.",
    anchors: [cast_os_stdlib::multicore_numa::cpu_topology_model::CpuTopologyModel],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
