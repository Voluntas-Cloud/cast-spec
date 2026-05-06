//! `split_brain_prevention` — avoid two primaries acting independently.

/// Sentinel for `split_brain_prevention`.
pub struct SplitBrainPrevention;

cast::concept! {
    name: "split_brain_prevention",
    summary: "Avoid two primaries acting independently. Quorum, \
              fencing tokens, or external arbiters ensure that at most \
              one party believes itself to be authoritative.",
    anchors: [cast_stdlib::reliability::split_brain_prevention::SplitBrainPrevention],
    tags: ["cast_stdlib", "reliability"],
}
