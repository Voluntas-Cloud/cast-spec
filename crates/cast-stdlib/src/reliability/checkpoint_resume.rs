//! `checkpoint_resume` — resume work from saved progress.

/// Sentinel for `checkpoint_resume`.
pub struct CheckpointResume;

cast::concept! {
    name: "checkpoint_resume",
    summary: "Resume work from saved progress. Long-running operations \
              periodically save state so a restart picks up where it \
              left off rather than starting over.",
    anchors: [cast_stdlib::reliability::checkpoint_resume::CheckpointResume],
    tags: ["cast_stdlib", "reliability"],
}
