//! `bsd_monolithic_kernel` — Unix-derived monolithic kernel family.

/// Sentinel for `bsd_monolithic_kernel`.
pub struct BsdMonolithicKernel;

cast::concept! {
    name: "bsd_monolithic_kernel",
    summary: "Unix-derived monolithic kernel family.",
    anchors: [cast_os_stdlib::kernel_families::bsd_monolithic_kernel::BsdMonolithicKernel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
