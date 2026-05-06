//! `post_deploy_verification` — verify system after change.

/// Sentinel for `post_deploy_verification`.
pub struct PostDeployVerification;

cast::concept! {
    name: "post_deploy_verification",
    summary: "Verify system after change. Smoke tests, key-metric \
              checks, synthetic probes — the deploy isn't 'done' \
              until the post-deploy verification passes.",
    anchors: [cast_stdlib::deployment::post_deploy_verification::PostDeployVerification],
    tags: ["cast_stdlib", "deployment"],
}
