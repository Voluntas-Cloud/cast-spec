//! `saga_transaction` — long-running transaction split into compensatable steps.

/// Sentinel for `saga_transaction`.
pub struct SagaTransaction;

cast::concept! {
    name: "saga_transaction",
    summary: "Long-running transaction split into compensatable steps. \
              Distributed transactions modeled as a sequence of \
              forward-then-undo actions.",
    anchors: [cast_stdlib::consistency::saga_transaction::SagaTransaction],
    tags: ["cast_stdlib", "consistency"],
}
