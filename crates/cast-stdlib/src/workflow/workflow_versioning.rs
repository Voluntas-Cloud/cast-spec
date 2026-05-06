//! `workflow_versioning` — running workflows keep their original definition.

/// Sentinel for `workflow_versioning`.
pub struct WorkflowVersioning;

cast::concept! {
    name: "workflow_versioning",
    summary: "Running workflows keep their original definition. Code \
              changes apply to new instances; in-flight workflows \
              continue against the version they started with so resume \
              and replay are deterministic.",
    anchors: [cast_stdlib::workflow::workflow_versioning::WorkflowVersioning],
    tags: ["cast_stdlib", "workflow"],
}
