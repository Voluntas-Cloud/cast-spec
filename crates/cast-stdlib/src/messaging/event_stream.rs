//! `event_stream` — ordered, append-only, replayable stream of facts.

/// Sentinel for `event_stream`.
pub struct EventStream;

cast::concept! {
    name: "event_stream",
    summary: "Ordered stream of facts. Append-only, replayable; consumers \
              track their own position.",
    anchors: [cast_stdlib::messaging::event_stream::EventStream],
    tags: ["cast_stdlib", "messaging"],
}
