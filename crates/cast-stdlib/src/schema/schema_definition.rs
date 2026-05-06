//! `schema_definition` — formal structure of data.

/// Sentinel for `schema_definition`.
pub struct SchemaDefinition;

cast::concept! {
    name: "schema_definition",
    summary: "Formal structure of data. Field names, types, optionality, \
              constraints — captured in a place that producers and \
              consumers both reference.",
    anchors: [cast_stdlib::schema::schema_definition::SchemaDefinition],
    tags: ["cast_stdlib", "schema"],
}
