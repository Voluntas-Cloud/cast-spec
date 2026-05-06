//! `webhook_delivery_system` — events are delivered to external systems reliably and securely.

/// Sentinel for `webhook_delivery_system`.
pub struct WebhookDeliverySystem;

cast::concept! {
    name: "webhook_delivery_system",
    summary: "Events are delivered to external systems reliably and \
              securely. Composes webhook_callback, \
              callback_signature_verification, retry_with_backoff, \
              dead_letter_queue, idempotency_key, \
              rate_limit_contract, event_message, and audit_log. \
              Used for payment callbacks, external notifications, \
              automation triggers, GitHub-style integrations, and \
              service-to-service callbacks.",
    anchors: [cast_stdlib::patterns::webhook_delivery_system::WebhookDeliverySystem],
    tags: ["cast_stdlib", "patterns"],
}
