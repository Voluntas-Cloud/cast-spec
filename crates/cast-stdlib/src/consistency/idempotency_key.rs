//! `idempotency_key` — unique key prevents duplicate effects.

/// Sentinel for `idempotency_key`.
pub struct IdempotencyKey;

cast::concept! {
    name: "idempotency_key",
    summary: "Unique key prevents duplicate effects. Receiver remembers \
              seen keys and returns the prior result instead of \
              re-applying.",
    anchors: [cast_stdlib::consistency::idempotency_key::IdempotencyKey],
    tags: ["cast_stdlib", "consistency"],
}
