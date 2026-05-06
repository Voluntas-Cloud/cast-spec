//! `cost_budget_policy` — resource usage constrained by cost.

/// Sentinel for `cost_budget_policy`.
pub struct CostBudgetPolicy;

cast::concept! {
    name: "cost_budget_policy",
    summary: "Resource usage constrained by cost. The scheduler reasons \
              about money the same way it reasons about CPU or memory; \
              jobs that would exceed a budget are rejected, downsized, \
              or queued.",
    anchors: [cast_stdlib::resources::cost_budget_policy::CostBudgetPolicy],
    tags: ["cast_stdlib", "resources"],
}
