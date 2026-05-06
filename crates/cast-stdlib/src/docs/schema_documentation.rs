//! `schema_documentation` — explain data structures.

/// Sentinel for `schema_documentation`.
pub struct SchemaDocumentation;

cast::concept! {
    name: "schema_documentation",
    summary: "Explain data structures: what each field means, what is \
              optional, what invariants hold. \"It's just a JSON blob\" \
              is not a schema, it's an admission of defeat.",
    anchors: [cast_stdlib::docs::schema_documentation::SchemaDocumentation],
    tags: ["cast_stdlib", "docs"],
}
