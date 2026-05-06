//! `self_healing_reconciler_control_plane` — continuously moves actual state toward desired state.

/// Sentinel for `self_healing_reconciler_control_plane`.
pub struct SelfHealingReconcilerControlPlane;

cast::concept! {
    name: "self_healing_reconciler_control_plane",
    summary: "A control system that continuously moves actual state \
              toward desired state. Composes desired_state_model, \
              actual_state_observation, drift_detection, \
              idempotent_operation, retry_with_backoff, \
              state_machine_lifecycle, status_condition, and \
              incremental_rebuild. Used for Kubernetes operators, \
              the Voluntas service reconciler, infrastructure \
              automation, device fleet management, and GitOps \
              controllers.",
    anchors: [cast_stdlib::patterns::self_healing_reconciler_control_plane::SelfHealingReconcilerControlPlane],
    tags: ["cast_stdlib", "patterns"],
}
