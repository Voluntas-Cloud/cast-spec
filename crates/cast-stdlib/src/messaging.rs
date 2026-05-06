//! Messaging, events & communication patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod at_least_once_delivery;
pub mod at_most_once_delivery;
pub mod backpressure_signal;
pub mod causation_id;
pub mod command_message;
pub mod correlation_id;
pub mod dead_letter_queue;
pub mod differential_publish;
pub mod event_message;
pub mod event_stream;
pub mod exactly_once_effect;
pub mod fire_and_forget_message;
pub mod line_oriented_protocol;
pub mod message_acknowledgement;
pub mod message_deduplication;
pub mod message_queue;
pub mod message_redelivery;
pub mod ordered_delivery;
pub mod partitioned_stream;
pub mod publish_subscribe;
pub mod query_message;
pub mod request_response;
pub mod side_effect_tee;

cast::concept! {
    name: "messaging",
    summary: "Umbrella for the messaging stdlib category. Messaging, \
              events & communication patterns.",
    anchors: [
        crate::messaging::at_least_once_delivery,
        crate::messaging::at_most_once_delivery,
        crate::messaging::backpressure_signal,
        crate::messaging::causation_id,
        crate::messaging::command_message,
        crate::messaging::correlation_id,
        crate::messaging::dead_letter_queue,
        crate::messaging::differential_publish,
        crate::messaging::event_message,
        crate::messaging::event_stream,
        crate::messaging::exactly_once_effect,
        crate::messaging::fire_and_forget_message,
        crate::messaging::line_oriented_protocol,
        crate::messaging::message_acknowledgement,
        crate::messaging::message_deduplication,
        crate::messaging::message_queue,
        crate::messaging::message_redelivery,
        crate::messaging::ordered_delivery,
        crate::messaging::partitioned_stream,
        crate::messaging::publish_subscribe,
        crate::messaging::query_message,
        crate::messaging::request_response,
        crate::messaging::side_effect_tee,
    ],
    tags: ["cast_stdlib", "messaging"],
}

/// Sentinel for the messaging stdlib group.
pub struct MessagingGroup;

cast::rule! {
    rule: "Assume duplicate, delayed, reordered, and lost messages unless your system proves otherwise.",
    why:  "Networks are just gossip with electricity. Optimism about \
           delivery is the leading cause of mysterious bugs.",
    governs: [cast_stdlib::messaging::MessagingGroup],
    tags: ["cast_stdlib", "messaging"],
}
