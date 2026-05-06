//! `dns_resolver_subsystem` — hostname resolution path.

/// Sentinel for `dns_resolver_subsystem`.
pub struct DnsResolverSubsystem;

cast::concept! {
    name: "dns_resolver_subsystem",
    summary: "hostname resolution path.",
    anchors: [cast_os_stdlib::networking::dns_resolver_subsystem::DnsResolverSubsystem],
    tags: ["cast_os_stdlib", "networking"],
}
