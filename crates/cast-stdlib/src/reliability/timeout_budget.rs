//! `timeout_budget` — every external call has a deadline.

/// Sentinel for `timeout_budget`.
pub struct TimeoutBudget;

cast::concept! {
    name: "timeout_budget",
    summary: "Every external call has a deadline. Without one, the \
              call inherits 'forever'; thread pools and resources \
              deplete waiting for a response that may never come.",
    anchors: [cast_stdlib::reliability::timeout_budget::TimeoutBudget],
    tags: ["cast_stdlib", "reliability"],
}
