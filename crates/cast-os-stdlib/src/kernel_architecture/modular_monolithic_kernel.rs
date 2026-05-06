//! `modular_monolithic_kernel` — monolithic kernel with loadable modules.

/// Sentinel for `modular_monolithic_kernel`.
pub struct ModularMonolithicKernel;

cast::concept! {
    name: "modular_monolithic_kernel",
    summary: "monolithic kernel with loadable modules.",
    anchors: [cast_os_stdlib::kernel_architecture::modular_monolithic_kernel::ModularMonolithicKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
