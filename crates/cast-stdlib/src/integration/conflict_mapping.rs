//! `conflict_mapping` — handle differing external/internal state.

/// Sentinel for `conflict_mapping`.
pub struct ConflictMapping;

cast::concept! {
    name: "conflict_mapping",
    summary: "Decide what happens when external and internal state \
              disagree — last-write-wins, internal-wins, prompt the \
              user, defer to a manual queue. Make the rule explicit \
              before sync runs in production, not after.",
    anchors: [cast_stdlib::integration::conflict_mapping::ConflictMapping],
    tags: ["cast_stdlib", "integration"],
}
