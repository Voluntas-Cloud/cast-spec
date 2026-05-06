//! `at_least_once_delivery` — message may arrive multiple times, never lost.

/// Sentinel for `at_least_once_delivery`.
pub struct AtLeastOnceDelivery;

cast::concept! {
    name: "at_least_once_delivery",
    summary: "Message may arrive multiple times. The transport guarantees \
              no message is lost; consumers must be idempotent.",
    anchors: [cast_stdlib::messaging::at_least_once_delivery::AtLeastOnceDelivery],
    tags: ["cast_stdlib", "messaging"],
}
