//! `atomic_operation` — all-or-nothing mutation.

/// Sentinel for `atomic_operation`.
pub struct AtomicOperation;

cast::concept! {
    name: "atomic_operation",
    summary: "All-or-nothing mutation. Either every effect is observable \
              or none of them are; partial states do not leak to readers.",
    anchors: [cast_stdlib::consistency::atomic_operation::AtomicOperation],
    tags: ["cast_stdlib", "consistency"],
}
