//! `redundancy` — duplicate critical capacity.

/// Sentinel for `redundancy`.
pub struct Redundancy;

cast::concept! {
    name: "redundancy",
    summary: "Duplicate critical capacity. Loss of one instance does \
              not cause loss of capability; cost is paying for spare \
              capacity that mostly sits idle.",
    anchors: [cast_stdlib::reliability::redundancy::Redundancy],
    tags: ["cast_stdlib", "reliability"],
}
