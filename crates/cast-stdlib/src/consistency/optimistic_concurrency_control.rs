//! `optimistic_concurrency_control` — assume no conflict, validate before commit.

/// Sentinel for `optimistic_concurrency_control`.
pub struct OptimisticConcurrencyControl;

cast::concept! {
    name: "optimistic_concurrency_control",
    summary: "Assume no conflict, validate before commit. Cheap when \
              contention is rare; retries when conflicts do happen.",
    anchors: [cast_stdlib::consistency::optimistic_concurrency_control::OptimisticConcurrencyControl],
    tags: ["cast_stdlib", "consistency"],
}
