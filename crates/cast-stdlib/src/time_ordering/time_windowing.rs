//! `time_windowing` — group events by time interval.

/// Sentinel for `time_windowing`.
pub struct TimeWindowing;

cast::concept! {
    name: "time_windowing",
    summary: "Group events into time intervals — tumbling, hopping, \
              session windows. The choice of window plus the choice \
              of timestamp (event vs. processing time) decides \
              whether your aggregates mean what users think they \
              mean.",
    anchors: [cast_stdlib::time_ordering::time_windowing::TimeWindowing],
    tags: ["cast_stdlib", "time_ordering"],
}
