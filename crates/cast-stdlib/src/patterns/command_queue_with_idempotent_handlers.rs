//! `command_queue_with_idempotent_handlers` — queued commands processed safely under retries.

/// Sentinel for `command_queue_with_idempotent_handlers`.
pub struct CommandQueueWithIdempotentHandlers;

cast::concept! {
    name: "command_queue_with_idempotent_handlers",
    summary: "Commands are queued and processed safely, even when \
              delivery happens more than once. Composes \
              message_queue, command_message, at_least_once_delivery, \
              idempotency_key, idempotent_operation, \
              dead_letter_queue, retry_with_backoff, and \
              poison_record_detection. Used for background jobs, \
              email sending, payment processing, image/build \
              pipelines, and cluster maintenance tasks.",
    anchors: [cast_stdlib::patterns::command_queue_with_idempotent_handlers::CommandQueueWithIdempotentHandlers],
    tags: ["cast_stdlib", "patterns"],
}
