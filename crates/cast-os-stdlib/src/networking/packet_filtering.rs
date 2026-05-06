//! `packet_filtering` — decide allowed/blocked traffic.

/// Sentinel for `packet_filtering`.
pub struct PacketFiltering;

cast::concept! {
    name: "packet_filtering",
    summary: "decide allowed/blocked traffic.",
    anchors: [cast_os_stdlib::networking::packet_filtering::PacketFiltering],
    tags: ["cast_os_stdlib", "networking"],
}
