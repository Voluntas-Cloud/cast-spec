//! `xnu_hybrid_kernel` — Apple kernel combining Mach, BSD, and I/O Kit ideas.

/// Sentinel for `xnu_hybrid_kernel`.
pub struct XnuHybridKernel;

cast::concept! {
    name: "xnu_hybrid_kernel",
    summary: "Apple kernel combining Mach, BSD, and I/O Kit ideas.",
    anchors: [cast_os_stdlib::kernel_families::xnu_hybrid_kernel::XnuHybridKernel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
