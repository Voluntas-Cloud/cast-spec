//! `eventual_consistency` — replicas converge over time.

/// Sentinel for `eventual_consistency`.
pub struct EventualConsistency;

cast::concept! {
    name: "eventual_consistency",
    summary: "Replicas converge over time. Reads may see stale data; \
              writes succeed without waiting for global agreement.",
    anchors: [cast_stdlib::consistency::eventual_consistency::EventualConsistency],
    tags: ["cast_stdlib", "consistency"],
}
