//! `disaster_recovery_plan` — restore service after major failure.

/// Sentinel for `disaster_recovery_plan`.
pub struct DisasterRecoveryPlan;

cast::concept! {
    name: "disaster_recovery_plan",
    summary: "Restore service after major failure. Documented, \
              practiced, and reviewed; survives the loss of an entire \
              site, region, or substrate.",
    anchors: [cast_stdlib::reliability::disaster_recovery_plan::DisasterRecoveryPlan],
    tags: ["cast_stdlib", "reliability"],
}
