//! `isolated_cpu_core` — CPU reserved for low-jitter workloads.

/// Sentinel for `isolated_cpu_core`.
pub struct IsolatedCpuCore;

cast::concept! {
    name: "isolated_cpu_core",
    summary: "CPU reserved for low-jitter workloads.",
    anchors: [cast_os_stdlib::realtime::isolated_cpu_core::IsolatedCpuCore],
    tags: ["cast_os_stdlib", "realtime"],
}
