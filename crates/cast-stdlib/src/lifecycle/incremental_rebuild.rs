//! `incremental_rebuild` — work scales to changed input only.

/// Sentinel for `incremental_rebuild`.
pub struct IncrementalRebuild;

cast::concept! {
    name: "incremental_rebuild",
    summary: "Work scales to changed input, not whole corpus. Depends \
              on stable per-input identity (typically a content hash) \
              and a parse cache keyed by that identity.",
    anchors: [cast_stdlib::lifecycle::incremental_rebuild::IncrementalRebuild],
    tags: ["cast_stdlib", "lifecycle"],
}
