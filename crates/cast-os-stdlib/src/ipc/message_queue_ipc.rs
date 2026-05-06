//! `message_queue_ipc` — queued structured messages.

/// Sentinel for `message_queue_ipc`.
pub struct MessageQueueIpc;

cast::concept! {
    name: "message_queue_ipc",
    summary: "queued structured messages.",
    anchors: [cast_os_stdlib::ipc::message_queue_ipc::MessageQueueIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
