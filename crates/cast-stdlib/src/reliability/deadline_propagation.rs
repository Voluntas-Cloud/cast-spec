//! `deadline_propagation` — downstream callers inherit the remaining time budget.

/// Sentinel for `deadline_propagation`.
pub struct DeadlinePropagation;

cast::concept! {
    name: "deadline_propagation",
    summary: "An operation's deadline travels with the call: each \
              downstream layer inherits the time remaining instead of \
              starting a fresh fixed timeout. Where `timeout_budget` \
              says every external call has a deadline, this concept \
              says every chained call shares the *same* deadline — so \
              a 500ms top-level budget can't be silently spent five \
              times by five layers each using a 500ms local timeout.",
    anchors: [cast_stdlib::reliability::deadline_propagation::DeadlinePropagation],
    tags: ["cast_stdlib", "reliability"],
}
