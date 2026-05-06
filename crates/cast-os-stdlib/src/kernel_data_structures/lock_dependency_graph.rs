//! `lock_dependency_graph` — graph used for lock-order debugging.

/// Sentinel for `lock_dependency_graph`.
pub struct LockDependencyGraph;

cast::concept! {
    name: "lock_dependency_graph",
    summary: "graph used for lock-order debugging.",
    anchors: [cast_os_stdlib::kernel_data_structures::lock_dependency_graph::LockDependencyGraph],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
