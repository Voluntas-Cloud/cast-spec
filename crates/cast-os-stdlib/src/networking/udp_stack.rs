//! `udp_stack` — datagram protocol implementation.

/// Sentinel for `udp_stack`.
pub struct UdpStack;

cast::concept! {
    name: "udp_stack",
    summary: "datagram protocol implementation.",
    anchors: [cast_os_stdlib::networking::udp_stack::UdpStack],
    tags: ["cast_os_stdlib", "networking"],
}
