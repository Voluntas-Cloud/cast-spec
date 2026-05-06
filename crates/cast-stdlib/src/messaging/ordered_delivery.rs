//! `ordered_delivery` — preserve sequence constraints.

/// Sentinel for `ordered_delivery`.
pub struct OrderedDelivery;

cast::concept! {
    name: "ordered_delivery",
    summary: "Preserve sequence constraints. Either globally ordered \
              (one consumer) or per-key ordered (partitioned).",
    anchors: [cast_stdlib::messaging::ordered_delivery::OrderedDelivery],
    tags: ["cast_stdlib", "messaging"],
}
