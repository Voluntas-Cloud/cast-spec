//! `type_1_hypervisor` — bare-metal hypervisor.

/// Sentinel for `type_1_hypervisor`.
pub struct Type1Hypervisor;

cast::concept! {
    name: "type_1_hypervisor",
    summary: "bare-metal hypervisor.",
    anchors: [cast_os_stdlib::virtualization::type_1_hypervisor::Type1Hypervisor],
    tags: ["cast_os_stdlib", "virtualization"],
}
