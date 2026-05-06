//! `capacity_planning` — estimate required resources.

/// Sentinel for `capacity_planning`.
pub struct CapacityPlanning;

cast::concept! {
    name: "capacity_planning",
    summary: "Estimate required resources. Based on growth projections, \
              traffic patterns, and historical headroom; informs \
              when to add capacity before you need it.",
    anchors: [cast_stdlib::performance::capacity_planning::CapacityPlanning],
    tags: ["cast_stdlib", "performance"],
}
