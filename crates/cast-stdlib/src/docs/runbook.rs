//! `runbook` — operational steps for known situations.

/// Sentinel for `runbook`.
pub struct Runbook;

cast::concept! {
    name: "runbook",
    summary: "Operational steps for known situations. A runbook is the \
              difference between \"oncall already knows what to do\" \
              and \"oncall is now reverse-engineering the system at \
              3am\".",
    anchors: [cast_stdlib::docs::runbook::Runbook],
    tags: ["cast_stdlib", "docs"],
}
