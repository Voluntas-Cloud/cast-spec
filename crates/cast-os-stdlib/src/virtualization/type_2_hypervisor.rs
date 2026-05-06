//! `type_2_hypervisor` — hosted hypervisor.

/// Sentinel for `type_2_hypervisor`.
pub struct Type2Hypervisor;

cast::concept! {
    name: "type_2_hypervisor",
    summary: "hosted hypervisor.",
    anchors: [cast_os_stdlib::virtualization::type_2_hypervisor::Type2Hypervisor],
    tags: ["cast_os_stdlib", "virtualization"],
}
