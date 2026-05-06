//! `message_bus_backbone` — a central communication fabric carries commands, events, heartbeats, and status.

/// Sentinel for `message_bus_backbone`.
pub struct MessageBusBackbone;

cast::concept! {
    name: "message_bus_backbone",
    summary: "A central communication fabric carries commands, \
              events, heartbeats, and status. Composes \
              publish_subscribe, message_queue, event_stream, \
              correlation_id, message_acknowledgement, \
              dead_letter_queue, backpressure_signal, and \
              ordered_delivery. Used for NATS-based platforms, \
              internal automation buses, service orchestration, \
              event-driven personal clouds, and agent coordination.",
    anchors: [cast_stdlib::patterns::message_bus_backbone::MessageBusBackbone],
    tags: ["cast_stdlib", "patterns"],
}
