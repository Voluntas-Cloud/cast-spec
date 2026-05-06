//! Kernel/user boundary mechanisms.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod context_switch_boundary;
pub mod interrupt_gate;
pub mod ioctl_control_interface;
pub mod kernel_abi;
pub mod kernel_space;
pub mod kernel_upcall;
pub mod privilege_ring_model;
pub mod stable_syscall_abi;
pub mod supervisor_mode;
pub mod syscall_dispatch_table;
pub mod system_call_interface;
pub mod trap_gate;
pub mod user_mode;
pub mod user_space;
pub mod user_space_driver_boundary;

cast::concept! {
    name: "kernel_user_boundary",
    summary: "Umbrella for the kernel_user_boundary stdlib category. \
              Kernel/user boundary mechanisms.",
    anchors: [
        crate::kernel_user_boundary::context_switch_boundary,
        crate::kernel_user_boundary::interrupt_gate,
        crate::kernel_user_boundary::ioctl_control_interface,
        crate::kernel_user_boundary::kernel_abi,
        crate::kernel_user_boundary::kernel_space,
        crate::kernel_user_boundary::kernel_upcall,
        crate::kernel_user_boundary::privilege_ring_model,
        crate::kernel_user_boundary::stable_syscall_abi,
        crate::kernel_user_boundary::supervisor_mode,
        crate::kernel_user_boundary::syscall_dispatch_table,
        crate::kernel_user_boundary::system_call_interface,
        crate::kernel_user_boundary::trap_gate,
        crate::kernel_user_boundary::user_mode,
        crate::kernel_user_boundary::user_space,
        crate::kernel_user_boundary::user_space_driver_boundary,
    ],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}

/// Sentinel for the kernel_user_boundary stdlib group.
pub struct KernelUserBoundaryGroup;
