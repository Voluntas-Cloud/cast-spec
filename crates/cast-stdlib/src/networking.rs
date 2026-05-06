//! Networking & service connectivity patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod address_translation;
pub mod connection_pooling;
pub mod dns_service_discovery;
pub mod egress_gateway;
pub mod forward_proxy;
pub mod http_routing;
pub mod ingress_gateway;
pub mod keepalive_probe;
pub mod load_balancer;
pub mod mutual_tls_transport;
pub mod nat_traversal;
pub mod network_policy;
pub mod registry_service_discovery;
pub mod reverse_proxy;
pub mod service_mesh;
pub mod tcp_proxying;
pub mod tls_passthrough;
pub mod tls_termination;
pub mod tunnel_transport;

cast::concept! {
    name: "networking",
    summary: "Umbrella for the networking stdlib category. Networking & \
              service connectivity patterns.",
    anchors: [
        crate::networking::address_translation,
        crate::networking::connection_pooling,
        crate::networking::dns_service_discovery,
        crate::networking::egress_gateway,
        crate::networking::forward_proxy,
        crate::networking::http_routing,
        crate::networking::ingress_gateway,
        crate::networking::keepalive_probe,
        crate::networking::load_balancer,
        crate::networking::mutual_tls_transport,
        crate::networking::nat_traversal,
        crate::networking::network_policy,
        crate::networking::registry_service_discovery,
        crate::networking::reverse_proxy,
        crate::networking::service_mesh,
        crate::networking::tcp_proxying,
        crate::networking::tls_passthrough,
        crate::networking::tls_termination,
        crate::networking::tunnel_transport,
    ],
    tags: ["cast_stdlib", "networking"],
}

/// Sentinel for the networking stdlib group.
pub struct NetworkingGroup;

cast::rule! {
    rule: "Document where TLS terminates.",
    why:  "Otherwise nobody knows who can see plaintext, which is \
           apparently \"architecture\" now. The diagram everyone draws \
           shows a single padlock; the wire reality is six hops, three \
           of them clear-text inside a trusted-zone fiction.",
    governs: [cast_stdlib::networking::NetworkingGroup],
    tags: ["cast_stdlib", "networking"],
}
