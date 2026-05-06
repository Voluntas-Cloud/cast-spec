//! `publish_subscribe` — many subscribers receive matching messages.

/// Sentinel for `publish_subscribe`.
pub struct PublishSubscribe;

cast::concept! {
    name: "publish_subscribe",
    summary: "Many subscribers receive matching messages. Publisher does \
              not know who is listening; subscribers receive what they \
              have subscribed to.",
    anchors: [cast_stdlib::messaging::publish_subscribe::PublishSubscribe],
    tags: ["cast_stdlib", "messaging"],
}
