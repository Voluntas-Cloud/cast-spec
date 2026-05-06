//! `product_type` — all fields inhabited simultaneously (record / aggregate).

/// Sentinel for `product_type`.
pub struct ProductType;

cast::concept! {
    name: "product_type",
    summary: "Values consist of all of a fixed set of named fields \
              simultaneously. Field access is total. Models 'all of \
              these together'. Contrast: sum_type.",
    anchors: [cast_stdlib::type_properties::product_type::ProductType],
    tags: ["cast_stdlib", "type_properties"],
}
