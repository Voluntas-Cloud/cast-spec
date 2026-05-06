//! `event_driven_architecture` — state changes communicated as events.

/// Sentinel for `event_driven_architecture`.
pub struct EventDrivenArchitecture;

cast::concept! {
    name: "event_driven_architecture",
    summary: "State changes communicated as events. Decoupled producers \
              and consumers; the event log is the integration boundary.",
    anchors: [cast_stdlib::architecture::event_driven_architecture::EventDrivenArchitecture],
    tags: ["cast_stdlib", "architecture"],
}
