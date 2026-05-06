//! `blackbox_monitoring` — observe from outside.

/// Sentinel for `blackbox_monitoring`.
pub struct BlackboxMonitoring;

cast::concept! {
    name: "blackbox_monitoring",
    summary: "Observe from outside. Make a request the way a real \
              client would; check the response. Detects the failures \
              that internal metrics rationalize away.",
    anchors: [cast_stdlib::observability::blackbox_monitoring::BlackboxMonitoring],
    tags: ["cast_stdlib", "observability"],
}
