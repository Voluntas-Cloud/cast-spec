//! `protocol_bridge` — translate between protocols.

/// Sentinel for `protocol_bridge`.
pub struct ProtocolBridge;

cast::concept! {
    name: "protocol_bridge",
    summary: "Translate between protocols — gRPC↔HTTP, MQTT↔NATS, \
              SOAP↔REST. The bridge owns the impedance mismatch so \
              callers don't have to; with no bridge, every caller \
              learns both protocols and reinvents the translation.",
    anchors: [cast_stdlib::integration::protocol_bridge::ProtocolBridge],
    tags: ["cast_stdlib", "integration"],
}
