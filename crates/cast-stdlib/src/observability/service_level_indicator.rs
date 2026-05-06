//! `service_level_indicator` — measured reliability metric.

/// Sentinel for `service_level_indicator`.
pub struct ServiceLevelIndicator;

cast::concept! {
    name: "service_level_indicator",
    summary: "Measured reliability metric. The actual percentage of \
              good events over total events for a defined dimension \
              (latency, availability, correctness).",
    anchors: [cast_stdlib::observability::service_level_indicator::ServiceLevelIndicator],
    tags: ["cast_stdlib", "observability"],
}
