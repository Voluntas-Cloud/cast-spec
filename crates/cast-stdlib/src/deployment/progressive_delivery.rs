//! `progressive_delivery` — automated staged rollout.

/// Sentinel for `progressive_delivery`.
pub struct ProgressiveDelivery;

cast::concept! {
    name: "progressive_delivery",
    summary: "Automated staged rollout. Canary widens, metrics steer, \
              rollback fires automatically if signals turn red — the \
              human's job is to set the policy, not to babysit the \
              rollout.",
    anchors: [cast_stdlib::deployment::progressive_delivery::ProgressiveDelivery],
    tags: ["cast_stdlib", "deployment"],
}
