//! `state_reconciliation` — repair drift between state sources.

/// Sentinel for `state_reconciliation`.
pub struct StateReconciliation;

cast::concept! {
    name: "state_reconciliation",
    summary: "Repair drift between sources that should agree. The \
              reconciler runs continuously, finds disagreements, and \
              applies a known winner. Without it, drift accumulates \
              quietly until something downstream notices.",
    anchors: [cast_stdlib::state_data::state_reconciliation::StateReconciliation],
    tags: ["cast_stdlib", "state_data"],
}
