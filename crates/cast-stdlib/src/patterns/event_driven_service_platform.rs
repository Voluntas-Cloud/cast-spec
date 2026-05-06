//! `event_driven_service_platform` — services communicate by publishing and consuming events.

/// Sentinel for `event_driven_service_platform`.
pub struct EventDrivenServicePlatform;

cast::concept! {
    name: "event_driven_service_platform",
    summary: "Services communicate by publishing and consuming \
              events. Composes publish_subscribe, event_stream, \
              event_message, correlation_id, causation_id, \
              message_acknowledgement, dead_letter_queue, and \
              eventual_consistency. Used for microservices, internal \
              buses, notification systems, automation triggers, and \
              domain-event propagation.",
    anchors: [cast_stdlib::patterns::event_driven_service_platform::EventDrivenServicePlatform],
    tags: ["cast_stdlib", "patterns"],
}
