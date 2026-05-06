//! `transactional_storage` — atomic multi-mutation commits.

/// Sentinel for `transactional_storage`.
pub struct TransactionalStorage;

cast::concept! {
    name: "transactional_storage",
    summary: "Storage operations with atomicity and rollback semantics. \
              Multiple mutations commit as one unit or not at all; \
              partial visibility never leaks to readers.",
    anchors: [cast_stdlib::storage::transactional_storage::TransactionalStorage],
    tags: ["cast_stdlib", "storage"],
}
