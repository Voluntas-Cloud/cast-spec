//! `kernel_address_space_layout_randomization` — randomize kernel memory layout.

/// Sentinel for `kernel_address_space_layout_randomization`.
pub struct KernelAddressSpaceLayoutRandomization;

cast::concept! {
    name: "kernel_address_space_layout_randomization",
    summary: "randomize kernel memory layout.",
    anchors: [cast_os_stdlib::memory_management::kernel_address_space_layout_randomization::KernelAddressSpaceLayoutRandomization],
    tags: ["cast_os_stdlib", "memory_management"],
}
