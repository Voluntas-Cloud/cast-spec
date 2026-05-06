//! `service_level_objective` — target reliability.

/// Sentinel for `service_level_objective`.
pub struct ServiceLevelObjective;

cast::concept! {
    name: "service_level_objective",
    summary: "Target reliability. The threshold the SLI must meet; \
              load-bearing input to error-budget decisions and to \
              the contract presented to consumers.",
    anchors: [cast_stdlib::observability::service_level_objective::ServiceLevelObjective],
    tags: ["cast_stdlib", "observability"],
}
