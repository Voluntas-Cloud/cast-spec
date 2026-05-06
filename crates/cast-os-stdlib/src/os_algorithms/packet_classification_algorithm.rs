//! `packet_classification_algorithm` — classify network packets.

/// Sentinel for `packet_classification_algorithm`.
pub struct PacketClassificationAlgorithm;

cast::concept! {
    name: "packet_classification_algorithm",
    summary: "classify network packets.",
    anchors: [cast_os_stdlib::os_algorithms::packet_classification_algorithm::PacketClassificationAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
