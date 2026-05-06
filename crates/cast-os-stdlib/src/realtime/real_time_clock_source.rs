//! `real_time_clock_source` — reliable clock for deadline handling.

/// Sentinel for `real_time_clock_source`.
pub struct RealTimeClockSource;

cast::concept! {
    name: "real_time_clock_source",
    summary: "reliable clock for deadline handling.",
    anchors: [cast_os_stdlib::realtime::real_time_clock_source::RealTimeClockSource],
    tags: ["cast_os_stdlib", "realtime"],
}
