//! `separation_kernel` — kernel focused on strict partitioning and isolation.

/// Sentinel for `separation_kernel`.
pub struct SeparationKernel;

cast::concept! {
    name: "separation_kernel",
    summary: "kernel focused on strict partitioning and isolation.",
    anchors: [cast_os_stdlib::kernel_architecture::separation_kernel::SeparationKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
