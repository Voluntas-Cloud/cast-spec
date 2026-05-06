//! `backward_compatible_schema` — new reader handles old data.

/// Sentinel for `backward_compatible_schema`.
pub struct BackwardCompatibleSchema;

cast::concept! {
    name: "backward_compatible_schema",
    summary: "New reader can handle old data. Old records remain \
              readable after the schema evolves.",
    anchors: [cast_stdlib::schema::backward_compatible_schema::BackwardCompatibleSchema],
    tags: ["cast_stdlib", "schema"],
}
