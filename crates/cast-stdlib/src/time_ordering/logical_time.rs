//! `logical_time` — ordering without relying on clocks.

/// Sentinel for `logical_time`.
pub struct LogicalTime;

cast::concept! {
    name: "logical_time",
    summary: "Order events without trusting clocks at all. Lamport \
              and vector clocks let nodes agree on \"happened before\" \
              even when their wall clocks disagree by minutes; \
              especially useful in systems where NTP isn't reliable.",
    anchors: [cast_stdlib::time_ordering::logical_time::LogicalTime],
    tags: ["cast_stdlib", "time_ordering"],
}
