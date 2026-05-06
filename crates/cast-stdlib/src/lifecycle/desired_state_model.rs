//! `desired_state_model` — declarative target state, decoupled from steps.

/// Sentinel for `desired_state_model`.
pub struct DesiredStateModel;

cast::concept! {
    name: "desired_state_model",
    summary: "User/system declares target state. Decoupled from \
              imperative steps to reach it — the reconciler figures \
              out the path.",
    anchors: [cast_stdlib::lifecycle::desired_state_model::DesiredStateModel],
    tags: ["cast_stdlib", "lifecycle"],
}
