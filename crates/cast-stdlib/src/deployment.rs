//! Deployment & release patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod automatic_rollback;
pub mod blue_green_deployment;
pub mod canary_release;
pub mod change_freeze;
pub mod dark_launch;
pub mod database_expand_contract;
pub mod deployment_ring;
pub mod manual_approval_gate;
pub mod migration_gate;
pub mod post_deploy_verification;
pub mod preflight_check;
pub mod progressive_delivery;
pub mod release_notes;
pub mod rolling_update;
pub mod shadow_traffic;
pub mod zero_downtime_deployment;

cast::concept! {
    name: "deployment",
    summary: "Umbrella for the deployment stdlib category. Deployment & \
              release patterns.",
    anchors: [
        crate::deployment::automatic_rollback,
        crate::deployment::blue_green_deployment,
        crate::deployment::canary_release,
        crate::deployment::change_freeze,
        crate::deployment::dark_launch,
        crate::deployment::database_expand_contract,
        crate::deployment::deployment_ring,
        crate::deployment::manual_approval_gate,
        crate::deployment::migration_gate,
        crate::deployment::post_deploy_verification,
        crate::deployment::preflight_check,
        crate::deployment::progressive_delivery,
        crate::deployment::release_notes,
        crate::deployment::rolling_update,
        crate::deployment::shadow_traffic,
        crate::deployment::zero_downtime_deployment,
    ],
    tags: ["cast_stdlib", "deployment"],
}

/// Sentinel for the deployment stdlib group.
pub struct DeploymentGroup;

cast::rule! {
    rule: "Deployment should be boring.",
    why:  "If deploying feels heroic, your process is broken and \
           wearing a tiny cape. Every heroic deploy is a sign of \
           an invariant the automation does not yet capture.",
    governs: [cast_stdlib::deployment::DeploymentGroup],
    tags: ["cast_stdlib", "deployment"],
}
