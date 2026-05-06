//! Networking stack concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod bridge_interface;
pub mod dns_resolver_subsystem;
pub mod ebpf_packet_processing;
pub mod firewall_rule_engine;
pub mod ip_routing_table;
pub mod linux_skb_buffer;
pub mod napi_polling_model;
pub mod netfilter_hooks;
pub mod network_driver_interface;
pub mod network_interface_abstraction;
pub mod network_namespace;
pub mod packet_buffer;
pub mod packet_filtering;
pub mod receive_side_scaling;
pub mod service_discovery_local;
pub mod socket_api;
pub mod tcp_congestion_control;
pub mod tcp_stack;
pub mod traffic_control_qdisc;
pub mod tun_tap_device;
pub mod udp_stack;
pub mod virtual_network_interface;
pub mod vpn_tunnel_interface;
pub mod windows_net_buffer_list;
pub mod xdp_fast_path;

cast::concept! {
    name: "networking",
    summary: "Umbrella for the networking stdlib category. Networking \
              stack concepts.",
    anchors: [
        crate::networking::bridge_interface,
        crate::networking::dns_resolver_subsystem,
        crate::networking::ebpf_packet_processing,
        crate::networking::firewall_rule_engine,
        crate::networking::ip_routing_table,
        crate::networking::linux_skb_buffer,
        crate::networking::napi_polling_model,
        crate::networking::netfilter_hooks,
        crate::networking::network_driver_interface,
        crate::networking::network_interface_abstraction,
        crate::networking::network_namespace,
        crate::networking::packet_buffer,
        crate::networking::packet_filtering,
        crate::networking::receive_side_scaling,
        crate::networking::service_discovery_local,
        crate::networking::socket_api,
        crate::networking::tcp_congestion_control,
        crate::networking::tcp_stack,
        crate::networking::traffic_control_qdisc,
        crate::networking::tun_tap_device,
        crate::networking::udp_stack,
        crate::networking::virtual_network_interface,
        crate::networking::vpn_tunnel_interface,
        crate::networking::windows_net_buffer_list,
        crate::networking::xdp_fast_path,
    ],
    tags: ["cast_os_stdlib", "networking"],
}

/// Sentinel for the networking stdlib group.
pub struct NetworkingGroup;
