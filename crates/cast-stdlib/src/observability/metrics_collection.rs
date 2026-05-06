//! `metrics_collection` — numeric measurements over time.

/// Sentinel for `metrics_collection`.
pub struct MetricsCollection;

cast::concept! {
    name: "metrics_collection",
    summary: "Numeric measurements over time. Counters, gauges, \
              histograms — aggregated and queryable, the substrate \
              for dashboards and alerts.",
    anchors: [cast_stdlib::observability::metrics_collection::MetricsCollection],
    tags: ["cast_stdlib", "observability"],
}
