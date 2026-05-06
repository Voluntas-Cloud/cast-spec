//! `dag_workflow` — acyclic dependency workflow.

/// Sentinel for `dag_workflow`.
pub struct DagWorkflow;

cast::concept! {
    name: "dag_workflow",
    summary: "Acyclic dependency workflow. The graph is a DAG by \
              construction; cycles are rejected before run-time so \
              progress is guaranteed to terminate if individual tasks \
              do.",
    anchors: [cast_stdlib::workflow::dag_workflow::DagWorkflow],
    tags: ["cast_stdlib", "workflow"],
}
