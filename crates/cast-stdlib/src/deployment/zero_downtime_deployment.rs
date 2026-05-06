//! `zero_downtime_deployment` — no visible interruption during change.

/// Sentinel for `zero_downtime_deployment`.
pub struct ZeroDowntimeDeployment;

cast::concept! {
    name: "zero_downtime_deployment",
    summary: "No visible interruption during change. Every contract \
              the deploy crosses — backward compatibility, graceful \
              shutdown, in-flight request draining — is what makes \
              this possible.",
    anchors: [cast_stdlib::deployment::zero_downtime_deployment::ZeroDowntimeDeployment],
    tags: ["cast_stdlib", "deployment"],
}
