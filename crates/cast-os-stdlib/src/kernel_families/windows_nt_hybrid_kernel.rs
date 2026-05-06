//! `windows_nt_hybrid_kernel` — hybrid object-based kernel architecture with executive services.

/// Sentinel for `windows_nt_hybrid_kernel`.
pub struct WindowsNtHybridKernel;

cast::concept! {
    name: "windows_nt_hybrid_kernel",
    summary: "hybrid object-based kernel architecture with executive \
               services.",
    anchors: [cast_os_stdlib::kernel_families::windows_nt_hybrid_kernel::WindowsNtHybridKernel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
