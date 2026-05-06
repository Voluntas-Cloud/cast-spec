//! `virtual_machine_monitor` — hypervisor component controlling guests.

/// Sentinel for `virtual_machine_monitor`.
pub struct VirtualMachineMonitor;

cast::concept! {
    name: "virtual_machine_monitor",
    summary: "hypervisor component controlling guests.",
    anchors: [cast_os_stdlib::virtualization::virtual_machine_monitor::VirtualMachineMonitor],
    tags: ["cast_os_stdlib", "virtualization"],
}
