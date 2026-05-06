//! `vpn_tunnel_interface` — encrypted tunnel as network interface.

/// Sentinel for `vpn_tunnel_interface`.
pub struct VpnTunnelInterface;

cast::concept! {
    name: "vpn_tunnel_interface",
    summary: "encrypted tunnel as network interface.",
    anchors: [cast_os_stdlib::networking::vpn_tunnel_interface::VpnTunnelInterface],
    tags: ["cast_os_stdlib", "networking"],
}
