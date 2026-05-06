//! Real-time OS concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod bounded_blocking;
pub mod cyclictest_latency_measurement;
pub mod deadline_miss_detection;
pub mod deterministic_latency;
pub mod firm_real_time_system;
pub mod hard_real_time_system;
pub mod high_resolution_timer;
pub mod interrupt_affinity_control;
pub mod interrupt_latency;
pub mod isolated_cpu_core;
pub mod lock_free_realtime_path;
pub mod memory_locking_for_realtime;
pub mod preempt_rt_kernel;
pub mod real_time_clock_source;
pub mod real_time_mutex;
pub mod rt_safe_driver_model;
pub mod scheduling_latency;
pub mod soft_real_time_system;
pub mod wait_free_realtime_path;
pub mod worst_case_execution_time;

cast::concept! {
    name: "realtime",
    summary: "Umbrella for the realtime stdlib category. Real-time OS \
              concepts.",
    anchors: [
        crate::realtime::bounded_blocking,
        crate::realtime::cyclictest_latency_measurement,
        crate::realtime::deadline_miss_detection,
        crate::realtime::deterministic_latency,
        crate::realtime::firm_real_time_system,
        crate::realtime::hard_real_time_system,
        crate::realtime::high_resolution_timer,
        crate::realtime::interrupt_affinity_control,
        crate::realtime::interrupt_latency,
        crate::realtime::isolated_cpu_core,
        crate::realtime::lock_free_realtime_path,
        crate::realtime::memory_locking_for_realtime,
        crate::realtime::preempt_rt_kernel,
        crate::realtime::real_time_clock_source,
        crate::realtime::real_time_mutex,
        crate::realtime::rt_safe_driver_model,
        crate::realtime::scheduling_latency,
        crate::realtime::soft_real_time_system,
        crate::realtime::wait_free_realtime_path,
        crate::realtime::worst_case_execution_time,
    ],
    tags: ["cast_os_stdlib", "realtime"],
}

/// Sentinel for the realtime stdlib group.
pub struct RealtimeGroup;
