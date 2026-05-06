//! Virtualization and hypervisors.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod confidential_vm;
pub mod device_emulation;
pub mod device_passthrough;
pub mod full_virtualization;
pub mod guest_os;
pub mod hardware_virtualization_extensions;
pub mod host_os;
pub mod hypercall_interface;
pub mod live_migration;
pub mod memory_ballooning;
pub mod microvm_architecture;
pub mod nested_virtualization;
pub mod paravirtualization;
pub mod snapshot_vm_state;
pub mod sr_iov_virtual_function;
pub mod type_1_hypervisor;
pub mod type_2_hypervisor;
pub mod virtio_device_model;
pub mod virtual_cpu_scheduler;
pub mod virtual_interrupt_injection;
pub mod virtual_machine_monitor;

cast::concept! {
    name: "virtualization",
    summary: "Umbrella for the virtualization stdlib category. \
              Virtualization and hypervisors.",
    anchors: [
        crate::virtualization::confidential_vm,
        crate::virtualization::device_emulation,
        crate::virtualization::device_passthrough,
        crate::virtualization::full_virtualization,
        crate::virtualization::guest_os,
        crate::virtualization::hardware_virtualization_extensions,
        crate::virtualization::host_os,
        crate::virtualization::hypercall_interface,
        crate::virtualization::live_migration,
        crate::virtualization::memory_ballooning,
        crate::virtualization::microvm_architecture,
        crate::virtualization::nested_virtualization,
        crate::virtualization::paravirtualization,
        crate::virtualization::snapshot_vm_state,
        crate::virtualization::sr_iov_virtual_function,
        crate::virtualization::type_1_hypervisor,
        crate::virtualization::type_2_hypervisor,
        crate::virtualization::virtio_device_model,
        crate::virtualization::virtual_cpu_scheduler,
        crate::virtualization::virtual_interrupt_injection,
        crate::virtualization::virtual_machine_monitor,
    ],
    tags: ["cast_os_stdlib", "virtualization"],
}

/// Sentinel for the virtualization stdlib group.
pub struct VirtualizationGroup;
