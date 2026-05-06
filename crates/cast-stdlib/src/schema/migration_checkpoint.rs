//! `migration_checkpoint` — safe point during schema transition.

/// Sentinel for `migration_checkpoint`.
pub struct MigrationCheckpoint;

cast::concept! {
    name: "migration_checkpoint",
    summary: "Safe point during schema transition. The system can \
              stop, restart, or roll back at the checkpoint without \
              leaving data in an intermediate state.",
    anchors: [cast_stdlib::schema::migration_checkpoint::MigrationCheckpoint],
    tags: ["cast_stdlib", "schema"],
}
