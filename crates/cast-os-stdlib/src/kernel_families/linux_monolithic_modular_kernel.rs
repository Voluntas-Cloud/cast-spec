//! `linux_monolithic_modular_kernel` — monolithic Unix-like kernel with dynamic modules.

/// Sentinel for `linux_monolithic_modular_kernel`.
pub struct LinuxMonolithicModularKernel;

cast::concept! {
    name: "linux_monolithic_modular_kernel",
    summary: "monolithic Unix-like kernel with dynamic modules.",
    anchors: [cast_os_stdlib::kernel_families::linux_monolithic_modular_kernel::LinuxMonolithicModularKernel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
