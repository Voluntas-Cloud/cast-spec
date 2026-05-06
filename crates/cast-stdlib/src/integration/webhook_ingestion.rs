//! `webhook_ingestion` — receive external event callbacks.

/// Sentinel for `webhook_ingestion`.
pub struct WebhookIngestion;

cast::concept! {
    name: "webhook_ingestion",
    summary: "Receive event callbacks from external systems. The \
              endpoint must verify signatures, accept replays \
              idempotently, and respond fast enough not to trigger \
              the sender's retry policy — all of which are easy to \
              get wrong.",
    anchors: [cast_stdlib::integration::webhook_ingestion::WebhookIngestion],
    tags: ["cast_stdlib", "integration"],
}
