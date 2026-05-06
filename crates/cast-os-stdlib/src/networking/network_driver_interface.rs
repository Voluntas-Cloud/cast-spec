//! `network_driver_interface` — OS-to-network-driver boundary.

/// Sentinel for `network_driver_interface`.
pub struct NetworkDriverInterface;

cast::concept! {
    name: "network_driver_interface",
    summary: "OS-to-network-driver boundary.",
    anchors: [cast_os_stdlib::networking::network_driver_interface::NetworkDriverInterface],
    tags: ["cast_os_stdlib", "networking"],
}
