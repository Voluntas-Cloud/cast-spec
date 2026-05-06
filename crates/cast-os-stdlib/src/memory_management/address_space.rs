//! `address_space` — process or kernel memory mapping domain.

/// Sentinel for `address_space`.
pub struct AddressSpace;

cast::concept! {
    name: "address_space",
    summary: "process or kernel memory mapping domain.",
    anchors: [cast_os_stdlib::memory_management::address_space::AddressSpace],
    tags: ["cast_os_stdlib", "memory_management"],
}
