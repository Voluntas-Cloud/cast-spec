//! `high_cardinality_metric_guard` — bound metric label space.

/// Sentinel for `high_cardinality_metric_guard`.
pub struct HighCardinalityMetricGuard;

cast::concept! {
    name: "high_cardinality_metric_guard",
    summary: "Avoid unbounded metric labels. User IDs, request IDs, \
              and free-form strings as label values explode the metric \
              cardinality and detonate the metrics backend.",
    anchors: [cast_stdlib::observability::high_cardinality_metric_guard::HighCardinalityMetricGuard],
    tags: ["cast_stdlib", "observability"],
}
