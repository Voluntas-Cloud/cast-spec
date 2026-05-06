//! `per_cpu_run_queue` — run queue local to CPU core.

/// Sentinel for `per_cpu_run_queue`.
pub struct PerCpuRunQueue;

cast::concept! {
    name: "per_cpu_run_queue",
    summary: "run queue local to CPU core.",
    anchors: [cast_os_stdlib::scheduling::per_cpu_run_queue::PerCpuRunQueue],
    tags: ["cast_os_stdlib", "scheduling"],
}
