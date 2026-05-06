//! `nested_virtualization` — VM running its own hypervisor.

/// Sentinel for `nested_virtualization`.
pub struct NestedVirtualization;

cast::concept! {
    name: "nested_virtualization",
    summary: "VM running its own hypervisor.",
    anchors: [cast_os_stdlib::virtualization::nested_virtualization::NestedVirtualization],
    tags: ["cast_os_stdlib", "virtualization"],
}
