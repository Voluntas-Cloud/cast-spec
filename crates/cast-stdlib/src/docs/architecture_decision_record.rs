//! `architecture_decision_record` — record decision, context, tradeoffs.

/// Sentinel for `architecture_decision_record`.
pub struct ArchitectureDecisionRecord;

cast::concept! {
    name: "architecture_decision_record",
    summary: "Record a decision together with its context and \
              tradeoffs. The format forces you to write down what you \
              considered and rejected, which is exactly the part that \
              gets lost otherwise.",
    anchors: [cast_stdlib::docs::architecture_decision_record::ArchitectureDecisionRecord],
    tags: ["cast_stdlib", "docs"],
}
