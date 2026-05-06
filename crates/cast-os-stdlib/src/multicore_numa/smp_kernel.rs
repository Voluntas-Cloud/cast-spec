//! `smp_kernel` — symmetric multiprocessing support.

/// Sentinel for `smp_kernel`.
pub struct SmpKernel;

cast::concept! {
    name: "smp_kernel",
    summary: "symmetric multiprocessing support.",
    anchors: [cast_os_stdlib::multicore_numa::smp_kernel::SmpKernel],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
