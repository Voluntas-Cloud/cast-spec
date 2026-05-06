//! `read_repair` — upgrade data when it is read.

/// Sentinel for `read_repair`.
pub struct ReadRepair;

cast::concept! {
    name: "read_repair",
    summary: "Upgrade data when it is read. Old records are migrated \
              lazily on first access rather than in a big batch — \
              spreads the migration cost across normal traffic.",
    anchors: [cast_stdlib::schema::read_repair::ReadRepair],
    tags: ["cast_stdlib", "schema"],
}
