//! `strong_consistency` — reads observe latest committed write.

/// Sentinel for `strong_consistency`.
pub struct StrongConsistency;

cast::concept! {
    name: "strong_consistency",
    summary: "Reads observe the latest committed write. Pays \
              availability and latency for the guarantee.",
    anchors: [cast_stdlib::consistency::strong_consistency::StrongConsistency],
    tags: ["cast_stdlib", "consistency"],
}
