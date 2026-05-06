//! `message_acknowledgement` — consumer confirms processing.

/// Sentinel for `message_acknowledgement`.
pub struct MessageAcknowledgement;

cast::concept! {
    name: "message_acknowledgement",
    summary: "Consumer confirms processing. Until ack, the broker holds \
              the message available for redelivery.",
    anchors: [cast_stdlib::messaging::message_acknowledgement::MessageAcknowledgement],
    tags: ["cast_stdlib", "messaging"],
}
