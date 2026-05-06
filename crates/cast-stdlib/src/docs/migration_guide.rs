//! `migration_guide` — explain how to move between versions.

/// Sentinel for `migration_guide`.
pub struct MigrationGuide;

cast::concept! {
    name: "migration_guide",
    summary: "Explain how to move from one version to another: what \
              changed, what to do about it, what the rollback path \
              looks like. The absence of one is how upgrades become \
              archaeology.",
    anchors: [cast_stdlib::docs::migration_guide::MigrationGuide],
    tags: ["cast_stdlib", "docs"],
}
