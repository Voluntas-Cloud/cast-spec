//! `split_brain_recovery` — safe recovery after partition.

/// Sentinel for `split_brain_recovery`.
pub struct SplitBrainRecovery;

cast::concept! {
    name: "split_brain_recovery",
    summary: "Safe recovery after partition. Once the partition heals, \
              the cluster reconciles divergent state without losing \
              writes from the surviving side.",
    anchors: [cast_stdlib::distributed::split_brain_recovery::SplitBrainRecovery],
    tags: ["cast_stdlib", "distributed"],
}
