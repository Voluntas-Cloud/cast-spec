//! `compensation_workflow` — recovery path for partial success.

/// Sentinel for `compensation_workflow`.
pub struct CompensationWorkflow;

cast::concept! {
    name: "compensation_workflow",
    summary: "Recovery path for partial success. When a multi-step \
              workflow fails halfway, registered compensators run in \
              reverse to undo the visible effects of completed steps.",
    anchors: [cast_stdlib::workflow::compensation_workflow::CompensationWorkflow],
    tags: ["cast_stdlib", "workflow"],
}
