//! `network_interface_abstraction` — OS representation of NIC.

/// Sentinel for `network_interface_abstraction`.
pub struct NetworkInterfaceAbstraction;

cast::concept! {
    name: "network_interface_abstraction",
    summary: "OS representation of NIC.",
    anchors: [cast_os_stdlib::networking::network_interface_abstraction::NetworkInterfaceAbstraction],
    tags: ["cast_os_stdlib", "networking"],
}
