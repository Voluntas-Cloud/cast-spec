//! `sum_type` — exactly one variant inhabited at a time (tagged union).

/// Sentinel for `sum_type`.
pub struct SumType;

cast::concept! {
    name: "sum_type",
    summary: "Values inhabit exactly one of a finite set of named \
              variants. Models 'one of these shapes' — lifecycle \
              states, error categories, parse-tree nodes. Contrast: \
              product_type.",
    anchors: [cast_stdlib::type_properties::sum_type::SumType],
    tags: ["cast_stdlib", "type_properties"],
}
