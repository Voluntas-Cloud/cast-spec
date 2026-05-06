//! `adaptive_io_scheduler` — I/O scheduling adapts to device/workload.

/// Sentinel for `adaptive_io_scheduler`.
pub struct AdaptiveIoScheduler;

cast::concept! {
    name: "adaptive_io_scheduler",
    summary: "I/O scheduling adapts to device/workload.",
    anchors: [cast_os_stdlib::self_adaptive_os::adaptive_io_scheduler::AdaptiveIoScheduler],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
