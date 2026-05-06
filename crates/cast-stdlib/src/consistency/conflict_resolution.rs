//! `conflict_resolution` — deterministic handling of competing writes.

/// Sentinel for `conflict_resolution`.
pub struct ConflictResolution;

cast::concept! {
    name: "conflict_resolution",
    summary: "Deterministic handling of competing writes. Strategies: \
              last-writer-wins, mergeable state, CRDT, escalate to user.",
    anchors: [cast_stdlib::consistency::conflict_resolution::ConflictResolution],
    tags: ["cast_stdlib", "consistency"],
}
