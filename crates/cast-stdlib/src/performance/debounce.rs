//! `debounce` — coalesce a burst of events into a single trailing action.

/// Sentinel for `debounce`.
pub struct Debounce;

cast::concept! {
    name: "debounce",
    summary: "Coalesce a burst of repeated events into a single \
              action. While events keep arriving for the same key \
              within a sliding window, defer the work; once the \
              stream has been quiet for the window's duration, fire \
              once. Pays the per-event cost a single time per logical \
              edit instead of N times for the same intent.",
    anchors: [cast_stdlib::performance::debounce::Debounce],
    tags: ["cast_stdlib", "performance"],
}
