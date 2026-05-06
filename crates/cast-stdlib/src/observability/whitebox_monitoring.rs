//! `whitebox_monitoring` — observe from inside.

/// Sentinel for `whitebox_monitoring`.
pub struct WhiteboxMonitoring;

cast::concept! {
    name: "whitebox_monitoring",
    summary: "Observe from inside. Internal counters, queue depths, \
              cache hit rates — the metrics only the application \
              itself can produce.",
    anchors: [cast_stdlib::observability::whitebox_monitoring::WhiteboxMonitoring],
    tags: ["cast_stdlib", "observability"],
}
