//! `lazy_migration` — migrate gradually, formats coexist.

/// Sentinel for `lazy_migration`.
pub struct LazyMigration;

cast::concept! {
    name: "lazy_migration",
    summary: "Migrate gradually instead of all at once. Old and new \
              formats coexist for a long time; pairs naturally with \
              read_repair and dual_write_migration.",
    anchors: [cast_stdlib::schema::lazy_migration::LazyMigration],
    tags: ["cast_stdlib", "schema"],
}
