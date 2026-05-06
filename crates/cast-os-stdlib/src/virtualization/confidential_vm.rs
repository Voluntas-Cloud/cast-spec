//! `confidential_vm` — guest protected from host/hypervisor inspection.

/// Sentinel for `confidential_vm`.
pub struct ConfidentialVm;

cast::concept! {
    name: "confidential_vm",
    summary: "guest protected from host/hypervisor inspection.",
    anchors: [cast_os_stdlib::virtualization::confidential_vm::ConfidentialVm],
    tags: ["cast_os_stdlib", "virtualization"],
}
