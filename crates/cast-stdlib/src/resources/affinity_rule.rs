//! `affinity_rule` — prefer/require placement near something.

/// Sentinel for `affinity_rule`.
pub struct AffinityRule;

cast::concept! {
    name: "affinity_rule",
    summary: "Prefer or require placement near something. Workloads \
              that share data, latency budgets, or hardware are kept \
              together; soft affinity is a hint, hard affinity is a \
              constraint that can leave work unschedulable.",
    anchors: [cast_stdlib::resources::affinity_rule::AffinityRule],
    tags: ["cast_stdlib", "resources"],
}
