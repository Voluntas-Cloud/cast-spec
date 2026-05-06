//! Consistency, transactions & concurrency patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod atomic_operation;
pub mod causal_consistency;
pub mod compare_and_swap;
pub mod compensating_action;
pub mod conflict_resolution;
pub mod crdt_state;
pub mod distributed_lock;
pub mod eventual_consistency;
pub mod idempotency_key;
pub mod last_writer_wins;
pub mod lease_based_lock;
pub mod mergeable_state;
pub mod optimistic_concurrency_control;
pub mod pessimistic_locking;
pub mod read_committed;
pub mod repeatable_read;
pub mod saga_transaction;
pub mod serializable_transaction;
pub mod snapshot_isolation;
pub mod strong_consistency;
pub mod transaction_boundary;

cast::concept! {
    name: "consistency",
    summary: "Umbrella for the consistency stdlib category. Consistency, \
              transactions & concurrency patterns.",
    anchors: [
        crate::consistency::atomic_operation,
        crate::consistency::causal_consistency,
        crate::consistency::compare_and_swap,
        crate::consistency::compensating_action,
        crate::consistency::conflict_resolution,
        crate::consistency::crdt_state,
        crate::consistency::distributed_lock,
        crate::consistency::eventual_consistency,
        crate::consistency::idempotency_key,
        crate::consistency::last_writer_wins,
        crate::consistency::lease_based_lock,
        crate::consistency::mergeable_state,
        crate::consistency::optimistic_concurrency_control,
        crate::consistency::pessimistic_locking,
        crate::consistency::read_committed,
        crate::consistency::repeatable_read,
        crate::consistency::saga_transaction,
        crate::consistency::serializable_transaction,
        crate::consistency::snapshot_isolation,
        crate::consistency::strong_consistency,
        crate::consistency::transaction_boundary,
    ],
    tags: ["cast_stdlib", "consistency"],
}

/// Sentinel for the consistency stdlib group.
pub struct ConsistencyGroup;

cast::rule! {
    rule: "Pick the weakest consistency model that preserves correctness.",
    why:  "Not vibes. Correctness. Stronger models cost availability \
           and latency you don't need to spend; weaker models are \
           often sufficient and dramatically cheaper.",
    governs: [cast_stdlib::consistency::ConsistencyGroup],
    tags: ["cast_stdlib", "consistency"],
}
