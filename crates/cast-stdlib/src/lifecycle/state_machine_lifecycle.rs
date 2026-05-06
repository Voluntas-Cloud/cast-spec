//! `state_machine_lifecycle` — explicit named states, enumerated transitions.

/// Sentinel for `state_machine_lifecycle`.
pub struct StateMachineLifecycle;

cast::concept! {
    name: "state_machine_lifecycle",
    summary: "Allowed transitions are explicit. Every state is named; \
              every transition is enumerated; invalid combinations \
              are unrepresentable, not just unlikely.",
    anchors: [cast_stdlib::lifecycle::state_machine_lifecycle::StateMachineLifecycle],
    tags: ["cast_stdlib", "lifecycle"],
}
