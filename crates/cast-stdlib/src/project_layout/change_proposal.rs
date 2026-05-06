//! `change_proposal` — structured design/change request.

/// Sentinel for `change_proposal`.
pub struct ChangeProposal;

cast::concept! {
    name: "change_proposal",
    summary: "A structured request for a non-trivial change — \
              design, motivation, alternatives, rollout. Forces the \
              author to write the parts that turn out to matter \
              before the discussion gets stuck on syntax.",
    anchors: [cast_stdlib::project_layout::change_proposal::ChangeProposal],
    tags: ["cast_stdlib", "project_layout"],
}
