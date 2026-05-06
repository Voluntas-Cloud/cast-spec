//! `manual_intervention_hook` — operator can resume/override.

/// Sentinel for `manual_intervention_hook`.
pub struct ManualInterventionHook;

cast::concept! {
    name: "manual_intervention_hook",
    summary: "Operator can resume or override. Stuck workflows expose a \
              way for a human to skip a step, edit pending state, or \
              force a transition — auditable, narrow, and visible \
              afterwards.",
    anchors: [cast_stdlib::workflow::manual_intervention_hook::ManualInterventionHook],
    tags: ["cast_stdlib", "workflow"],
}
