//! `runbook_linking` — alerts link to mitigation docs.

/// Sentinel for `runbook_linking`.
pub struct RunbookLinking;

cast::concept! {
    name: "runbook_linking",
    summary: "Alerts link to mitigation docs. The 3am responder gets \
              not just the symptom but the documented response; \
              shrinks time-to-action when minds are foggy.",
    anchors: [cast_stdlib::observability::runbook_linking::RunbookLinking],
    tags: ["cast_stdlib", "observability"],
}
