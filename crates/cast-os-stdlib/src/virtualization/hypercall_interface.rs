//! `hypercall_interface` — guest-to-hypervisor call boundary.

/// Sentinel for `hypercall_interface`.
pub struct HypercallInterface;

cast::concept! {
    name: "hypercall_interface",
    summary: "guest-to-hypervisor call boundary.",
    anchors: [cast_os_stdlib::virtualization::hypercall_interface::HypercallInterface],
    tags: ["cast_os_stdlib", "virtualization"],
}
