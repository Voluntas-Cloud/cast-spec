//! `schema_registry` — central catalog of schemas and versions.

/// Sentinel for `schema_registry`.
pub struct SchemaRegistry;

cast::concept! {
    name: "schema_registry",
    summary: "Central catalog of schemas and versions. Producers and \
              consumers look up schemas by ID; lifecycle (deprecation, \
              removal) is administered here.",
    anchors: [cast_stdlib::schema::schema_registry::SchemaRegistry],
    tags: ["cast_stdlib", "schema"],
}
