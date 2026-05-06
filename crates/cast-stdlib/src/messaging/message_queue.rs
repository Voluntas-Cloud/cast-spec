//! `message_queue` — messages consumed by workers, one per message.

/// Sentinel for `message_queue`.
pub struct MessageQueue;

cast::concept! {
    name: "message_queue",
    summary: "Messages consumed by workers. Each message is processed \
              by one worker; the queue smooths over producer/consumer \
              rate differences.",
    anchors: [cast_stdlib::messaging::message_queue::MessageQueue],
    tags: ["cast_stdlib", "messaging"],
}
