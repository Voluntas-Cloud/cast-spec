//! `exactly_once_effect` — at-least-once delivery + idempotent application.

/// Sentinel for `exactly_once_effect`.
pub struct ExactlyOnceEffect;

cast::concept! {
    name: "exactly_once_effect",
    summary: "Achieved through idempotency, not fairy dust. The transport \
              delivers at-least-once; the application is idempotent; \
              the effect is observed once.",
    anchors: [cast_stdlib::messaging::exactly_once_effect::ExactlyOnceEffect],
    tags: ["cast_stdlib", "messaging"],
}
