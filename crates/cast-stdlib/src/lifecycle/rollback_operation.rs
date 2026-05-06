//! `rollback_operation` — return to a previous known state.

/// Sentinel for `rollback_operation`.
pub struct RollbackOperation;

cast::concept! {
    name: "rollback_operation",
    summary: "Return to a previous known state. Requires that the \
              previous state was preserved; not all forward changes \
              are safely reversible.",
    anchors: [cast_stdlib::lifecycle::rollback_operation::RollbackOperation],
    tags: ["cast_stdlib", "lifecycle"],
}
