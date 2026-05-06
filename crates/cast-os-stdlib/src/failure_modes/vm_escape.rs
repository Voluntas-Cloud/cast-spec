//! `vm_escape` — guest breaks into hypervisor/host.

/// Sentinel for `vm_escape`.
pub struct VmEscape;

cast::concept! {
    name: "vm_escape",
    summary: "guest breaks into hypervisor/host.",
    anchors: [cast_os_stdlib::failure_modes::vm_escape::VmEscape],
    tags: ["cast_os_stdlib", "failure_modes"],
}
