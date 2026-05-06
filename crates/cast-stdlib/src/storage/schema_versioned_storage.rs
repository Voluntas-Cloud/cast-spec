//! `schema_versioned_storage` — persisted records carry schema metadata.

/// Sentinel for `schema_versioned_storage`.
pub struct SchemaVersionedStorage;

cast::concept! {
    name: "schema_versioned_storage",
    summary: "Persisted data includes schema/version metadata so a \
              reader can distinguish formats. Substrate for safe \
              schema evolution.",
    anchors: [cast_stdlib::storage::schema_versioned_storage::SchemaVersionedStorage],
    tags: ["cast_stdlib", "storage"],
}
