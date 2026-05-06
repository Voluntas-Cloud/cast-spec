//! `capability_kernel` — kernel authority is expressed through capabilities.

/// Sentinel for `capability_kernel`.
pub struct CapabilityKernel;

cast::concept! {
    name: "capability_kernel",
    summary: "kernel authority is expressed through capabilities.",
    anchors: [cast_os_stdlib::kernel_architecture::capability_kernel::CapabilityKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
