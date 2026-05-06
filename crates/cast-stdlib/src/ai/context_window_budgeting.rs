//! `context_window_budgeting` — manage limited prompt space.

/// Sentinel for `context_window_budgeting`.
pub struct ContextWindowBudgeting;

cast::concept! {
    name: "context_window_budgeting",
    summary: "Manage the finite prompt space across system, history, \
              retrieved context, and tool output. \"Just put more in \
              the prompt\" is how latency, cost, and degraded \
              attention all get worse simultaneously.",
    anchors: [cast_stdlib::ai::context_window_budgeting::ContextWindowBudgeting],
    tags: ["cast_stdlib", "ai"],
}
