//! `decision_log` — chronological design/history record.

/// Sentinel for `decision_log`.
pub struct DecisionLog;

cast::concept! {
    name: "decision_log",
    summary: "Chronological record of decisions, in order. Different \
              from architecture_decision_record in granularity: the \
              log is the timeline, the ADR is the deep-dive on a \
              specific choice.",
    anchors: [cast_stdlib::docs::decision_log::DecisionLog],
    tags: ["cast_stdlib", "docs"],
}
