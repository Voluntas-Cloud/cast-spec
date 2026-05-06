//! `dns_service_discovery` — find services via DNS.

/// Sentinel for `dns_service_discovery`.
pub struct DnsServiceDiscovery;

cast::concept! {
    name: "dns_service_discovery",
    summary: "Find services via DNS. Clients resolve a name to a set of \
              endpoints; the resolver and TTL bound how fast topology \
              changes propagate.",
    anchors: [cast_stdlib::networking::dns_service_discovery::DnsServiceDiscovery],
    tags: ["cast_stdlib", "networking"],
}
