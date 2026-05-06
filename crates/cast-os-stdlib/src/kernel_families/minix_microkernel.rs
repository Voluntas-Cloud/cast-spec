//! `minix_microkernel` — educational and reliability-focused microkernel lineage.

/// Sentinel for `minix_microkernel`.
pub struct MinixMicrokernel;

cast::concept! {
    name: "minix_microkernel",
    summary: "educational and reliability-focused microkernel lineage.",
    anchors: [cast_os_stdlib::kernel_families::minix_microkernel::MinixMicrokernel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
