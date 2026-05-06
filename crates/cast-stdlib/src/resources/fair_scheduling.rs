//! `fair_scheduling` — distribute resources equitably.

/// Sentinel for `fair_scheduling`.
pub struct FairScheduling;

cast::concept! {
    name: "fair_scheduling",
    summary: "Distribute resources equitably. Each tenant or queue gets \
              a defined share of capacity; the scheduler's notion of \
              \"fair\" is what the operator declared, not what users \
              feel.",
    anchors: [cast_stdlib::resources::fair_scheduling::FairScheduling],
    tags: ["cast_stdlib", "resources"],
}
