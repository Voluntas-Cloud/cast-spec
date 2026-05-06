//! Scheduling concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod completely_fair_scheduler;
pub mod cooperative_scheduling;
pub mod cpu_affinity;
pub mod deadline_scheduling;
pub mod earliest_deadline_first;
pub mod energy_aware_scheduling;
pub mod fair_share_scheduler;
pub mod latency_sensitive_scheduling;
pub mod load_balancing_scheduler;
pub mod multilevel_feedback_queue;
pub mod numa_aware_scheduling;
pub mod per_cpu_run_queue;
pub mod preemptive_scheduling;
pub mod priority_ceiling_protocol;
pub mod priority_inheritance;
pub mod priority_inversion;
pub mod priority_scheduler;
pub mod rate_monotonic_scheduling;
pub mod real_time_priority_class;
pub mod round_robin_scheduling;
pub mod run_queue;
pub mod scheduler_class;
pub mod scheduler_tick;
pub mod thermal_aware_scheduling;
pub mod throughput_oriented_scheduling;
pub mod tickless_kernel;
pub mod time_sharing_scheduler;
pub mod work_stealing_scheduler;

cast::concept! {
    name: "scheduling",
    summary: "Umbrella for the scheduling stdlib category. Scheduling \
              concepts.",
    anchors: [
        crate::scheduling::completely_fair_scheduler,
        crate::scheduling::cooperative_scheduling,
        crate::scheduling::cpu_affinity,
        crate::scheduling::deadline_scheduling,
        crate::scheduling::earliest_deadline_first,
        crate::scheduling::energy_aware_scheduling,
        crate::scheduling::fair_share_scheduler,
        crate::scheduling::latency_sensitive_scheduling,
        crate::scheduling::load_balancing_scheduler,
        crate::scheduling::multilevel_feedback_queue,
        crate::scheduling::numa_aware_scheduling,
        crate::scheduling::per_cpu_run_queue,
        crate::scheduling::preemptive_scheduling,
        crate::scheduling::priority_ceiling_protocol,
        crate::scheduling::priority_inheritance,
        crate::scheduling::priority_inversion,
        crate::scheduling::priority_scheduler,
        crate::scheduling::rate_monotonic_scheduling,
        crate::scheduling::real_time_priority_class,
        crate::scheduling::round_robin_scheduling,
        crate::scheduling::run_queue,
        crate::scheduling::scheduler_class,
        crate::scheduling::scheduler_tick,
        crate::scheduling::thermal_aware_scheduling,
        crate::scheduling::throughput_oriented_scheduling,
        crate::scheduling::tickless_kernel,
        crate::scheduling::time_sharing_scheduler,
        crate::scheduling::work_stealing_scheduler,
    ],
    tags: ["cast_os_stdlib", "scheduling"],
}

/// Sentinel for the scheduling stdlib group.
pub struct SchedulingGroup;
