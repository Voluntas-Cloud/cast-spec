//! `packet_buffer` — kernel network packet structure.

/// Sentinel for `packet_buffer`.
pub struct PacketBuffer;

cast::concept! {
    name: "packet_buffer",
    summary: "kernel network packet structure.",
    anchors: [cast_os_stdlib::networking::packet_buffer::PacketBuffer],
    tags: ["cast_os_stdlib", "networking"],
}
