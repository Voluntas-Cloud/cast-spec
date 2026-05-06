//! `event_message` — record that something happened.

/// Sentinel for `event_message`.
pub struct EventMessage;

cast::concept! {
    name: "event_message",
    summary: "Record that something happened. Past tense, immutable, \
              broadcast to whoever cares.",
    anchors: [cast_stdlib::messaging::event_message::EventMessage],
    tags: ["cast_stdlib", "messaging"],
}
