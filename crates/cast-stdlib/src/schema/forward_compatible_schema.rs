//! `forward_compatible_schema` — old reader tolerates new data.

/// Sentinel for `forward_compatible_schema`.
pub struct ForwardCompatibleSchema;

cast::concept! {
    name: "forward_compatible_schema",
    summary: "Old reader can tolerate new data. Producers can add new \
              fields without breaking deployed consumers that haven't \
              been updated.",
    anchors: [cast_stdlib::schema::forward_compatible_schema::ForwardCompatibleSchema],
    tags: ["cast_stdlib", "schema"],
}
