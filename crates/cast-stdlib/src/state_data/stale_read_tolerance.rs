//! `stale_read_tolerance` — define acceptable oldness.

/// Sentinel for `stale_read_tolerance`.
pub struct StaleReadTolerance;

cast::concept! {
    name: "stale_read_tolerance",
    summary: "Define how stale a read is allowed to be before the \
              feature breaks. \"As fresh as possible\" is not an \
              answer; \"up to 5 seconds for the dashboard, must be \
              live for the checkout\" is one.",
    anchors: [cast_stdlib::state_data::stale_read_tolerance::StaleReadTolerance],
    tags: ["cast_stdlib", "state_data"],
}
