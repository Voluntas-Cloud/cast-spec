//! `event_timeline` — chronological view of system behavior.

/// Sentinel for `event_timeline`.
pub struct EventTimeline;

cast::concept! {
    name: "event_timeline",
    summary: "Chronological view of system behavior. Deploys, alerts, \
              config changes, and notable events on one axis — the \
              first thing to look at during an incident.",
    anchors: [cast_stdlib::observability::event_timeline::EventTimeline],
    tags: ["cast_stdlib", "observability"],
}
