//! `schema_first_api` — schema defines contract before implementation.

/// Sentinel for `schema_first_api`.
pub struct SchemaFirstApi;

cast::concept! {
    name: "schema_first_api",
    summary: "Schema defines contract before implementation. \
              Implementations on either side derive from the schema; \
              drift is impossible by construction.",
    anchors: [cast_stdlib::api::schema_first_api::SchemaFirstApi],
    tags: ["cast_stdlib", "api"],
}
