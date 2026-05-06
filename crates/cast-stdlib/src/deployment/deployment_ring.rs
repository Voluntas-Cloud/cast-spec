//! `deployment_ring` — release to increasingly broad groups.

/// Sentinel for `deployment_ring`.
pub struct DeploymentRing;

cast::concept! {
    name: "deployment_ring",
    summary: "Release to increasingly broad groups. Internal users \
              first, then early adopters, then general availability — \
              each ring catches the bugs the previous one missed.",
    anchors: [cast_stdlib::deployment::deployment_ring::DeploymentRing],
    tags: ["cast_stdlib", "deployment"],
}
