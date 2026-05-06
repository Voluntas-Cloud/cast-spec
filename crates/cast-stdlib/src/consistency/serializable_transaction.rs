//! `serializable_transaction` — strongest isolation, outcome as if sequential.

/// Sentinel for `serializable_transaction`.
pub struct SerializableTransaction;

cast::concept! {
    name: "serializable_transaction",
    summary: "Strongest isolation; outcome as if sequential. Concurrent \
              transactions appear to have run one after the other.",
    anchors: [cast_stdlib::consistency::serializable_transaction::SerializableTransaction],
    tags: ["cast_stdlib", "consistency"],
}
