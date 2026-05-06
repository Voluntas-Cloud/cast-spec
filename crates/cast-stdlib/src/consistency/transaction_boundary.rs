//! `transaction_boundary` — explicit scope of atomic work.

/// Sentinel for `transaction_boundary`.
pub struct TransactionBoundary;

cast::concept! {
    name: "transaction_boundary",
    summary: "Explicit scope of atomic work. Operations inside commit \
              together; outside is best-effort.",
    anchors: [cast_stdlib::consistency::transaction_boundary::TransactionBoundary],
    tags: ["cast_stdlib", "consistency"],
}
