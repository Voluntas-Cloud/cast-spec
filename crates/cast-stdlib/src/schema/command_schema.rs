//! `command_schema` — structured representation of requested actions.

/// Sentinel for `command_schema`.
pub struct CommandSchema;

cast::concept! {
    name: "command_schema",
    summary: "Structured representation of requested actions. Imperative, \
              may be rejected, typically carries who-asked and \
              what-they-want.",
    anchors: [cast_stdlib::schema::command_schema::CommandSchema],
    tags: ["cast_stdlib", "schema"],
}
