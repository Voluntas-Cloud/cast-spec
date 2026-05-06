//! `bridge_interface` — L2 forwarding inside OS.

/// Sentinel for `bridge_interface`.
pub struct BridgeInterface;

cast::concept! {
    name: "bridge_interface",
    summary: "L2 forwarding inside OS.",
    anchors: [cast_os_stdlib::networking::bridge_interface::BridgeInterface],
    tags: ["cast_os_stdlib", "networking"],
}
