//! Kernel architecture styles.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod capability_kernel;
pub mod distributed_os_kernel;
pub mod exokernel_architecture;
pub mod hybrid_kernel_architecture;
pub mod library_os;
pub mod message_passing_kernel;
pub mod microkernel_architecture;
pub mod modular_monolithic_kernel;
pub mod monolithic_kernel;
pub mod nanokernel_architecture;
pub mod object_based_kernel;
pub mod policy_mechanism_split_kernel;
pub mod real_time_kernel;
pub mod rump_kernel;
pub mod separation_kernel;
pub mod service_oriented_os;
pub mod unikernel_architecture;

cast::concept! {
    name: "kernel_architecture",
    summary: "Umbrella for the kernel_architecture stdlib category. Kernel \
              architecture styles.",
    anchors: [
        crate::kernel_architecture::capability_kernel,
        crate::kernel_architecture::distributed_os_kernel,
        crate::kernel_architecture::exokernel_architecture,
        crate::kernel_architecture::hybrid_kernel_architecture,
        crate::kernel_architecture::library_os,
        crate::kernel_architecture::message_passing_kernel,
        crate::kernel_architecture::microkernel_architecture,
        crate::kernel_architecture::modular_monolithic_kernel,
        crate::kernel_architecture::monolithic_kernel,
        crate::kernel_architecture::nanokernel_architecture,
        crate::kernel_architecture::object_based_kernel,
        crate::kernel_architecture::policy_mechanism_split_kernel,
        crate::kernel_architecture::real_time_kernel,
        crate::kernel_architecture::rump_kernel,
        crate::kernel_architecture::separation_kernel,
        crate::kernel_architecture::service_oriented_os,
        crate::kernel_architecture::unikernel_architecture,
    ],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}

/// Sentinel for the kernel_architecture stdlib group.
pub struct KernelArchitectureGroup;
