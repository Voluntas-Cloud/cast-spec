//! `webhook_callback` — server calls client on event.

/// Sentinel for `webhook_callback`.
pub struct WebhookCallback;

cast::concept! {
    name: "webhook_callback",
    summary: "Server calls client on event. Inverts the polling pattern; \
              the server pushes when something interesting happens.",
    anchors: [cast_stdlib::api::webhook_callback::WebhookCallback],
    tags: ["cast_stdlib", "api"],
}
