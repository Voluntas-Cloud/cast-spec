//! `distributed_os_kernel` — OS abstractions span multiple machines.

/// Sentinel for `distributed_os_kernel`.
pub struct DistributedOsKernel;

cast::concept! {
    name: "distributed_os_kernel",
    summary: "OS abstractions span multiple machines.",
    anchors: [cast_os_stdlib::kernel_architecture::distributed_os_kernel::DistributedOsKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
