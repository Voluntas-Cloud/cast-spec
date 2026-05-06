//! `canary_release` — expose change to small subset first.

/// Sentinel for `canary_release`.
pub struct CanaryRelease;

cast::concept! {
    name: "canary_release",
    summary: "Expose change to small subset first. A few percent of \
              traffic exercises the new code; if metrics stay green, \
              the rollout widens; if not, the canary is rolled back \
              before full exposure.",
    anchors: [cast_stdlib::deployment::canary_release::CanaryRelease],
    tags: ["cast_stdlib", "deployment"],
}
