//! `compensating_action` — undo or offset a completed action.

/// Sentinel for `compensating_action`.
pub struct CompensatingAction;

cast::concept! {
    name: "compensating_action",
    summary: "Undo or offset a completed action. The saga's undo step; \
              not a true rollback because side effects may have been \
              observed.",
    anchors: [cast_stdlib::consistency::compensating_action::CompensatingAction],
    tags: ["cast_stdlib", "consistency"],
}
