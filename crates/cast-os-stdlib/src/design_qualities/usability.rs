//! `usability` — humans can operate it without needing a ritual candle.

/// Sentinel for `usability`.
pub struct Usability;

cast::concept! {
    name: "usability",
    summary: "humans can operate it without needing a ritual candle.",
    anchors: [cast_os_stdlib::design_qualities::usability::Usability],
    tags: ["cast_os_stdlib", "design_qualities"],
}
