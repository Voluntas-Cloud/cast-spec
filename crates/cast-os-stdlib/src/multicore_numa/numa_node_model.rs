//! `numa_node_model` — memory locality domains.

/// Sentinel for `numa_node_model`.
pub struct NumaNodeModel;

cast::concept! {
    name: "numa_node_model",
    summary: "memory locality domains.",
    anchors: [cast_os_stdlib::multicore_numa::numa_node_model::NumaNodeModel],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
