//! Lifecycle, mutation & change patterns.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only the
//! group sentinel and the category rule.

pub mod actual_state_observation;
pub mod create_update_delete_lifecycle;
pub mod deprecation_lifecycle;
pub mod desired_state_model;
pub mod drift_detection;
pub mod feature_lifecycle;
pub mod graceful_degradation;
pub mod hard_delete;
pub mod hot_swap;
pub mod idempotent_operation;
pub mod incremental_rebuild;
pub mod maintenance_window;
pub mod reconciliation_loop;
pub mod resource_finalizer;
pub mod rollback_operation;
pub mod rollforward_operation;
pub mod soft_delete;
pub mod state_machine_lifecycle;
pub mod structured_cancellation;
pub mod tombstone_record;

cast::concept! {
    name: "lifecycle",
    summary: "Umbrella for the lifecycle stdlib category. Lifecycle, \
              mutation & change patterns.",
    anchors: [
        crate::lifecycle::actual_state_observation,
        crate::lifecycle::create_update_delete_lifecycle,
        crate::lifecycle::deprecation_lifecycle,
        crate::lifecycle::desired_state_model,
        crate::lifecycle::drift_detection,
        crate::lifecycle::feature_lifecycle,
        crate::lifecycle::graceful_degradation,
        crate::lifecycle::hard_delete,
        crate::lifecycle::hot_swap,
        crate::lifecycle::idempotent_operation,
        crate::lifecycle::incremental_rebuild,
        crate::lifecycle::maintenance_window,
        crate::lifecycle::reconciliation_loop,
        crate::lifecycle::resource_finalizer,
        crate::lifecycle::rollback_operation,
        crate::lifecycle::rollforward_operation,
        crate::lifecycle::soft_delete,
        crate::lifecycle::state_machine_lifecycle,
        crate::lifecycle::structured_cancellation,
        crate::lifecycle::tombstone_record,
    ],
    tags: ["cast_stdlib", "lifecycle"],
}

/// Sentinel for the lifecycle stdlib group.
pub struct LifecycleGroup;

cast::rule! {
    rule: "Model lifecycle as a state machine, not a collection of booleans.",
    why:  "Otherwise every boolean becomes a tiny lawsuit against \
           your future self — invalid combinations multiply silently \
           and the system slips into states no one designed for.",
    governs: [cast_stdlib::lifecycle::LifecycleGroup],
    tags: ["cast_stdlib", "lifecycle"],
}
