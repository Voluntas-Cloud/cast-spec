//! `commit_convention` — structured commit messages.

/// Sentinel for `commit_convention`.
pub struct CommitConvention;

cast::concept! {
    name: "commit_convention",
    summary: "Structured commit messages — type, scope, body, \
              footers. Conventional history feeds changelogs, \
              release notes, and bisect; ad-hoc \"wip\" history \
              feeds nothing but regret.",
    anchors: [cast_stdlib::project_layout::commit_convention::CommitConvention],
    tags: ["cast_stdlib", "project_layout"],
}
