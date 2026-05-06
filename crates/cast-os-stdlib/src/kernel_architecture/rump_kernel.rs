//! `rump_kernel` — reuse kernel components as user-space libraries.

/// Sentinel for `rump_kernel`.
pub struct RumpKernel;

cast::concept! {
    name: "rump_kernel",
    summary: "reuse kernel components as user-space libraries.",
    anchors: [cast_os_stdlib::kernel_architecture::rump_kernel::RumpKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
