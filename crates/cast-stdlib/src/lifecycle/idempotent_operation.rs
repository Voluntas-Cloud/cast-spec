//! `idempotent_operation` — retryable without harm.

/// Sentinel for `idempotent_operation`.
pub struct IdempotentOperation;

cast::concept! {
    name: "idempotent_operation",
    summary: "Can be retried without harm. N applications have the \
              same effect as one. Critical under any boundary that \
              can drop or duplicate messages.",
    anchors: [cast_stdlib::lifecycle::idempotent_operation::IdempotentOperation],
    tags: ["cast_stdlib", "lifecycle"],
}
