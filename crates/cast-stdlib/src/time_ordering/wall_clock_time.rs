//! `wall_clock_time` — real-world timestamp.

/// Sentinel for `wall_clock_time`.
pub struct WallClockTime;

cast::concept! {
    name: "wall_clock_time",
    summary: "A timestamp tied to civil time. Useful for humans, \
              terrible for measuring durations: clocks jump for NTP, \
              DST, and leap seconds. Don't compute intervals from \
              two wall-clock readings.",
    anchors: [cast_stdlib::time_ordering::wall_clock_time::WallClockTime],
    tags: ["cast_stdlib", "time_ordering"],
}
