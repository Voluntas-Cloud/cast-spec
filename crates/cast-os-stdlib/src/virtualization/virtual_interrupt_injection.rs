//! `virtual_interrupt_injection` — hypervisor delivers virtual interrupts.

/// Sentinel for `virtual_interrupt_injection`.
pub struct VirtualInterruptInjection;

cast::concept! {
    name: "virtual_interrupt_injection",
    summary: "hypervisor delivers virtual interrupts.",
    anchors: [cast_os_stdlib::virtualization::virtual_interrupt_injection::VirtualInterruptInjection],
    tags: ["cast_os_stdlib", "virtualization"],
}
