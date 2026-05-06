//! `task_graph` — work modeled as dependent nodes.

/// Sentinel for `task_graph`.
pub struct TaskGraph;

cast::concept! {
    name: "task_graph",
    summary: "Work modeled as dependent nodes. Edges encode \
              must-finish-first; the engine schedules whatever has its \
              prerequisites met, in parallel where the graph allows.",
    anchors: [cast_stdlib::workflow::task_graph::TaskGraph],
    tags: ["cast_stdlib", "workflow"],
}
