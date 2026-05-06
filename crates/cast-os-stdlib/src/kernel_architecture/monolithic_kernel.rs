//! `monolithic_kernel` — most OS services run in kernel space.

/// Sentinel for `monolithic_kernel`.
pub struct MonolithicKernel;

cast::concept! {
    name: "monolithic_kernel",
    summary: "most OS services run in kernel space.",
    anchors: [cast_os_stdlib::kernel_architecture::monolithic_kernel::MonolithicKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
