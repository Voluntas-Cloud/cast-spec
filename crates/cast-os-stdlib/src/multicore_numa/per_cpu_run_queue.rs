//! `per_cpu_run_queue` — runnable tasks per CPU.

/// Sentinel for `per_cpu_run_queue`.
pub struct PerCpuRunQueue;

cast::concept! {
    name: "per_cpu_run_queue",
    summary: "runnable tasks per CPU.",
    anchors: [cast_os_stdlib::multicore_numa::per_cpu_run_queue::PerCpuRunQueue],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
