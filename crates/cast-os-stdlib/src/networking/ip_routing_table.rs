//! `ip_routing_table` — routing decision structure.

/// Sentinel for `ip_routing_table`.
pub struct IpRoutingTable;

cast::concept! {
    name: "ip_routing_table",
    summary: "routing decision structure.",
    anchors: [cast_os_stdlib::networking::ip_routing_table::IpRoutingTable],
    tags: ["cast_os_stdlib", "networking"],
}
