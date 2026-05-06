//! `node_capability_advertisement` — node reports available resources/features.

/// Sentinel for `node_capability_advertisement`.
pub struct NodeCapabilityAdvertisement;

cast::concept! {
    name: "node_capability_advertisement",
    summary: "node reports available resources/features.",
    anchors: [cast_os_stdlib::distributed_os::node_capability_advertisement::NodeCapabilityAdvertisement],
    tags: ["cast_os_stdlib", "distributed_os"],
}
