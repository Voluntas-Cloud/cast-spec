//! `hybrid_kernel_architecture` — microkernel-inspired design with many services still in kernel space.

/// Sentinel for `hybrid_kernel_architecture`.
pub struct HybridKernelArchitecture;

cast::concept! {
    name: "hybrid_kernel_architecture",
    summary: "microkernel-inspired design with many services still in \
               kernel space.",
    anchors: [cast_os_stdlib::kernel_architecture::hybrid_kernel_architecture::HybridKernelArchitecture],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
