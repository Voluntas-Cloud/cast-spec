//! `pessimistic_locking` — lock first, prevent conflicts.

/// Sentinel for `pessimistic_locking`.
pub struct PessimisticLocking;

cast::concept! {
    name: "pessimistic_locking",
    summary: "Prevent conflicts by locking first. Serializes contended \
              access; trades concurrency for simplicity.",
    anchors: [cast_stdlib::consistency::pessimistic_locking::PessimisticLocking],
    tags: ["cast_stdlib", "consistency"],
}
