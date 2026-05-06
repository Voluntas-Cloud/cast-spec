//! Self-adaptive and autonomic OS concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod adaptive_cache_management;
pub mod adaptive_io_scheduler;
pub mod adaptive_memory_pressure_response;
pub mod adaptive_prefetch_window;
pub mod adaptive_readahead;
pub mod anomaly_detection_in_kernel_metrics;
pub mod autonomic_resource_manager;
pub mod control_loop_os_management;
pub mod dynamic_tick_adjustment;
pub mod human_override_for_adaptation;
pub mod ml_guided_scheduling;
pub mod policy_feedback_loop;
pub mod policy_guarded_self_adaptation;
pub mod power_adaptive_execution;
pub mod predictive_prefetching;
pub mod self_healing_os;
pub mod self_monitoring_os;
pub mod self_optimizing_scheduler;
pub mod self_protecting_os;
pub mod self_tuning_os;
pub mod thermal_self_adaptation;
pub mod workload_classification;

cast::concept! {
    name: "self_adaptive_os",
    summary: "Umbrella for the self_adaptive_os stdlib category. \
              Self-adaptive and autonomic OS concepts.",
    anchors: [
        crate::self_adaptive_os::adaptive_cache_management,
        crate::self_adaptive_os::adaptive_io_scheduler,
        crate::self_adaptive_os::adaptive_memory_pressure_response,
        crate::self_adaptive_os::adaptive_prefetch_window,
        crate::self_adaptive_os::adaptive_readahead,
        crate::self_adaptive_os::anomaly_detection_in_kernel_metrics,
        crate::self_adaptive_os::autonomic_resource_manager,
        crate::self_adaptive_os::control_loop_os_management,
        crate::self_adaptive_os::dynamic_tick_adjustment,
        crate::self_adaptive_os::human_override_for_adaptation,
        crate::self_adaptive_os::ml_guided_scheduling,
        crate::self_adaptive_os::policy_feedback_loop,
        crate::self_adaptive_os::policy_guarded_self_adaptation,
        crate::self_adaptive_os::power_adaptive_execution,
        crate::self_adaptive_os::predictive_prefetching,
        crate::self_adaptive_os::self_healing_os,
        crate::self_adaptive_os::self_monitoring_os,
        crate::self_adaptive_os::self_optimizing_scheduler,
        crate::self_adaptive_os::self_protecting_os,
        crate::self_adaptive_os::self_tuning_os,
        crate::self_adaptive_os::thermal_self_adaptation,
        crate::self_adaptive_os::workload_classification,
    ],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}

/// Sentinel for the self_adaptive_os stdlib group.
pub struct SelfAdaptiveOsGroup;
