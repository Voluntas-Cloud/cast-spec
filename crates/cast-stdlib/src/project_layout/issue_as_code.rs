//! `issue_as_code` — issues stored in repository.

/// Sentinel for `issue_as_code`.
pub struct IssueAsCode;

cast::concept! {
    name: "issue_as_code",
    summary: "Issues live in the repository, version-controlled \
              alongside the code they describe. Travels with branches, \
              survives platform migrations, and can be reviewed in a \
              diff like everything else.",
    anchors: [cast_stdlib::project_layout::issue_as_code::IssueAsCode],
    tags: ["cast_stdlib", "project_layout"],
}
