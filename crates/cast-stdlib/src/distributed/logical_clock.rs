//! `logical_clock` — order events without wall-clock time.

/// Sentinel for `logical_clock`.
pub struct LogicalClock;

cast::concept! {
    name: "logical_clock",
    summary: "Order events without wall-clock time. A counter that \
              advances on each event; orders causally-related events \
              correctly without trusting node clocks.",
    anchors: [cast_stdlib::distributed::logical_clock::LogicalClock],
    tags: ["cast_stdlib", "distributed"],
}
