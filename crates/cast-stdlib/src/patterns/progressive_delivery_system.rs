//! `progressive_delivery_system` — changes are rolled out gradually with verification and rollback.

/// Sentinel for `progressive_delivery_system`.
pub struct ProgressiveDeliverySystem;

cast::concept! {
    name: "progressive_delivery_system",
    summary: "Changes are rolled out gradually with verification \
              and rollback. Composes canary_release, rolling_update, \
              blue_green_deployment, post_deploy_verification, \
              automatic_rollback, feature_flag, health_check, and \
              service_level_indicator. Used for service deployment, \
              mobile/backend rollout, cluster upgrades, AI model \
              rollout, and risky feature releases.",
    anchors: [cast_stdlib::patterns::progressive_delivery_system::ProgressiveDeliverySystem],
    tags: ["cast_stdlib", "patterns"],
}
