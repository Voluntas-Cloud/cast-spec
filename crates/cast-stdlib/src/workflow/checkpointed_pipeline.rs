//! `checkpointed_pipeline` — pipeline saves progress between stages.

/// Sentinel for `checkpointed_pipeline`.
pub struct CheckpointedPipeline;

cast::concept! {
    name: "checkpointed_pipeline",
    summary: "Pipeline saves progress between stages. Restart resumes \
              from the last successful checkpoint; expensive earlier \
              stages do not have to be redone every time a later one \
              flakes.",
    anchors: [cast_stdlib::workflow::checkpointed_pipeline::CheckpointedPipeline],
    tags: ["cast_stdlib", "workflow"],
}
