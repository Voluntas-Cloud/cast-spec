//! `value_type` — structurally compared, freely cloneable, no hidden mutable state.

/// Sentinel for `value_type`.
pub struct ValueType;

cast::concept! {
    name: "value_type",
    summary: "Equality is structural; copies are interchangeable; no \
              hidden mutable state, identity, or external resource. \
              Safe to copy, store, hash, serialise. Contrast: \
              resource_handle.",
    anchors: [cast_stdlib::type_properties::value_type::ValueType],
    tags: ["cast_stdlib", "type_properties"],
}
