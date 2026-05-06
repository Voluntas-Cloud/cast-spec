//! `anti_affinity_rule` — avoid co-placement.

/// Sentinel for `anti_affinity_rule`.
pub struct AntiAffinityRule;

cast::concept! {
    name: "anti_affinity_rule",
    summary: "Avoid co-placement. Replicas of the same service are \
              spread across failure domains so one bad host or rack \
              cannot take them all down at once.",
    anchors: [cast_stdlib::resources::anti_affinity_rule::AntiAffinityRule],
    tags: ["cast_stdlib", "resources"],
}
