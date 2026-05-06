//! `access_review` — periodically review who has access.

/// Sentinel for `access_review`.
pub struct AccessReview;

cast::concept! {
    name: "access_review",
    summary: "Periodically review who has access. Grants are revisited \
              on a cadence; people who left teams, roles, or the \
              company lose access by default rather than by remembering \
              to ask.",
    anchors: [cast_stdlib::privacy::access_review::AccessReview],
    tags: ["cast_stdlib", "privacy"],
}
