//! `interrupt_routing_algorithm` — map interrupts to CPUs.

/// Sentinel for `interrupt_routing_algorithm`.
pub struct InterruptRoutingAlgorithm;

cast::concept! {
    name: "interrupt_routing_algorithm",
    summary: "map interrupts to CPUs.",
    anchors: [cast_os_stdlib::os_algorithms::interrupt_routing_algorithm::InterruptRoutingAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
