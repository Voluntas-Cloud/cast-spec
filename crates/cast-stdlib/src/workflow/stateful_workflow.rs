//! `stateful_workflow` — workflow persists progress.

/// Sentinel for `stateful_workflow`.
pub struct StatefulWorkflow;

cast::concept! {
    name: "stateful_workflow",
    summary: "Workflow persists progress. Each step's result is durably \
              recorded; restart resumes from the last completed step \
              instead of re-running everything from the top.",
    anchors: [cast_stdlib::workflow::stateful_workflow::StatefulWorkflow],
    tags: ["cast_stdlib", "workflow"],
}
