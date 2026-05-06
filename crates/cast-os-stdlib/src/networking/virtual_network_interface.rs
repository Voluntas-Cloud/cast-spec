//! `virtual_network_interface` — software network interface.

/// Sentinel for `virtual_network_interface`.
pub struct VirtualNetworkInterface;

cast::concept! {
    name: "virtual_network_interface",
    summary: "software network interface.",
    anchors: [cast_os_stdlib::networking::virtual_network_interface::VirtualNetworkInterface],
    tags: ["cast_os_stdlib", "networking"],
}
