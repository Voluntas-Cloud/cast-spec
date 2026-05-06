//! `at_most_once_delivery` — may be lost but not duplicated.

/// Sentinel for `at_most_once_delivery`.
pub struct AtMostOnceDelivery;

cast::concept! {
    name: "at_most_once_delivery",
    summary: "Message may be lost but not duplicated. Useful when \
              duplicate processing is more expensive than missed \
              processing.",
    anchors: [cast_stdlib::messaging::at_most_once_delivery::AtMostOnceDelivery],
    tags: ["cast_stdlib", "messaging"],
}
