//! `schema_migration` — controlled change from one schema to another.

/// Sentinel for `schema_migration`.
pub struct SchemaMigration;

cast::concept! {
    name: "schema_migration",
    summary: "Controlled change from one schema to another. Includes \
              the procedure for moving existing data, not just the new \
              shape.",
    anchors: [cast_stdlib::schema::schema_migration::SchemaMigration],
    tags: ["cast_stdlib", "schema"],
}
