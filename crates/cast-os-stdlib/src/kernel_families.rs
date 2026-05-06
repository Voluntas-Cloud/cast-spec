//! Real-world kernel families.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod bsd_monolithic_kernel;
pub mod inferno_os_model;
pub mod linux_monolithic_modular_kernel;
pub mod linux_with_user_space_control_planes;
pub mod minix_microkernel;
pub mod plan9_distributed_os_model;
pub mod qnx_microkernel;
pub mod redox_microkernel_os;
pub mod windows_nt_hybrid_kernel;
pub mod windows_subsystem_model;
pub mod xnu_hybrid_kernel;
pub mod zircon_kernel;

cast::concept! {
    name: "kernel_families",
    summary: "Umbrella for the kernel_families stdlib category. Real-world \
              kernel families.",
    anchors: [
        crate::kernel_families::bsd_monolithic_kernel,
        crate::kernel_families::inferno_os_model,
        crate::kernel_families::linux_monolithic_modular_kernel,
        crate::kernel_families::linux_with_user_space_control_planes,
        crate::kernel_families::minix_microkernel,
        crate::kernel_families::plan9_distributed_os_model,
        crate::kernel_families::qnx_microkernel,
        crate::kernel_families::redox_microkernel_os,
        crate::kernel_families::windows_nt_hybrid_kernel,
        crate::kernel_families::windows_subsystem_model,
        crate::kernel_families::xnu_hybrid_kernel,
        crate::kernel_families::zircon_kernel,
    ],
    tags: ["cast_os_stdlib", "kernel_families"],
}

/// Sentinel for the kernel_families stdlib group.
pub struct KernelFamiliesGroup;
