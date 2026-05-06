//! `mergeable_state` — concurrent changes can be merged.

/// Sentinel for `mergeable_state`.
pub struct MergeableState;

cast::concept! {
    name: "mergeable_state",
    summary: "Concurrent changes can be merged. Requires the application \
              to define a merge function for each conflicting field.",
    anchors: [cast_stdlib::consistency::mergeable_state::MergeableState],
    tags: ["cast_stdlib", "consistency"],
}
