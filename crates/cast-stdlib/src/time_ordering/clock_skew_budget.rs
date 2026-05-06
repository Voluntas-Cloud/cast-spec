//! `clock_skew_budget` — tolerated time difference.

/// Sentinel for `clock_skew_budget`.
pub struct ClockSkewBudget;

cast::concept! {
    name: "clock_skew_budget",
    summary: "How much disagreement between clocks the protocol \
              tolerates. Lease durations, token expiry, and \
              window-closing watermarks all need to budget for \
              skew; without an explicit budget, you're picking one \
              by accident.",
    anchors: [cast_stdlib::time_ordering::clock_skew_budget::ClockSkewBudget],
    tags: ["cast_stdlib", "time_ordering"],
}
