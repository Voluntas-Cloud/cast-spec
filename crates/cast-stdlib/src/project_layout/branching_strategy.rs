//! `branching_strategy` — rules for source control flow.

/// Sentinel for `branching_strategy`.
pub struct BranchingStrategy;

cast::concept! {
    name: "branching_strategy",
    summary: "Rules for how branches are created, merged, and \
              deleted. The strategy decides where unfinished work \
              lives and how it reaches users; pick one explicitly \
              instead of letting it emerge.",
    anchors: [cast_stdlib::project_layout::branching_strategy::BranchingStrategy],
    tags: ["cast_stdlib", "project_layout"],
}
