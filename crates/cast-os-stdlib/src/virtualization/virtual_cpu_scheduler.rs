//! `virtual_cpu_scheduler` — schedule vCPUs on physical CPUs.

/// Sentinel for `virtual_cpu_scheduler`.
pub struct VirtualCpuScheduler;

cast::concept! {
    name: "virtual_cpu_scheduler",
    summary: "schedule vCPUs on physical CPUs.",
    anchors: [cast_os_stdlib::virtualization::virtual_cpu_scheduler::VirtualCpuScheduler],
    tags: ["cast_os_stdlib", "virtualization"],
}
