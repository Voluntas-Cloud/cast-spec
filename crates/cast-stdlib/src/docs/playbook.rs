//! `playbook` — broader response process.

/// Sentinel for `playbook`.
pub struct Playbook;

cast::concept! {
    name: "playbook",
    summary: "A broader response process than a runbook: who is \
              involved, how decisions are made, what the steps are. \
              Used for incidents, escalations, and recurring \
              cross-team work.",
    anchors: [cast_stdlib::docs::playbook::Playbook],
    tags: ["cast_stdlib", "docs"],
}
