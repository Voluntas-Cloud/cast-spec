//! `message_redelivery` — unacknowledged messages retried.

/// Sentinel for `message_redelivery`.
pub struct MessageRedelivery;

cast::concept! {
    name: "message_redelivery",
    summary: "Unacknowledged messages are retried. Combined with \
              idempotency, the substrate of at-least-once delivery.",
    anchors: [cast_stdlib::messaging::message_redelivery::MessageRedelivery],
    tags: ["cast_stdlib", "messaging"],
}
