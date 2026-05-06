//! `self_optimizing_scheduler` — scheduler tunes policies based on workload.

/// Sentinel for `self_optimizing_scheduler`.
pub struct SelfOptimizingScheduler;

cast::concept! {
    name: "self_optimizing_scheduler",
    summary: "scheduler tunes policies based on workload.",
    anchors: [cast_os_stdlib::self_adaptive_os::self_optimizing_scheduler::SelfOptimizingScheduler],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
