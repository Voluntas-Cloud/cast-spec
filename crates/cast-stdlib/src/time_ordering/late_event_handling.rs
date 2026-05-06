//! `late_event_handling` — process delayed events correctly.

/// Sentinel for `late_event_handling`.
pub struct LateEventHandling;

cast::concept! {
    name: "late_event_handling",
    summary: "Decide what to do with events that arrive after their \
              window closed. Drop, side-channel, or restate? Each \
              has costs; \"silently miscount\" is almost never the \
              right one, even though it's the default.",
    anchors: [cast_stdlib::time_ordering::late_event_handling::LateEventHandling],
    tags: ["cast_stdlib", "time_ordering"],
}
