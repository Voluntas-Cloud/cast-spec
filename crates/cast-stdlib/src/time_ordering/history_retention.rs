//! `history_retention` — how much past state is kept.

/// Sentinel for `history_retention`.
pub struct HistoryRetention;

cast::concept! {
    name: "history_retention",
    summary: "How much of the past is preserved, and for how long. \
              Drives debugging, audit, and \"what did the dashboard \
              say last Tuesday\" — all things that look optional \
              right up until they aren't.",
    anchors: [cast_stdlib::time_ordering::history_retention::HistoryRetention],
    tags: ["cast_stdlib", "time_ordering"],
}
