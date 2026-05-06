//! `error_budget` — allowed unreliability before slowing change.

/// Sentinel for `error_budget`.
pub struct ErrorBudget;

cast::concept! {
    name: "error_budget",
    summary: "Allowed unreliability before slowing change. The gap \
              between the SLO and 100%; once spent, the system tightens \
              up rather than absorbing more risk.",
    anchors: [cast_stdlib::observability::error_budget::ErrorBudget],
    tags: ["cast_stdlib", "observability"],
}
