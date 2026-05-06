//! `event_schema` — structured representation of facts/events.

/// Sentinel for `event_schema`.
pub struct EventSchema;

cast::concept! {
    name: "event_schema",
    summary: "Structured representation of facts/events. Past tense, \
              immutable, typically carries who/when/what.",
    anchors: [cast_stdlib::schema::event_schema::EventSchema],
    tags: ["cast_stdlib", "schema"],
}
