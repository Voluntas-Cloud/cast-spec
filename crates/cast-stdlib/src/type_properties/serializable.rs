//! `serializable` — encodable to bytes that fully capture the value.

/// Sentinel for `serializable`.
pub struct Serializable;

cast::concept! {
    name: "serializable",
    summary: "Encodable to / decodable from a transport format (JSON, \
              CBOR, bincode, …) preserving meaning. The bytes ARE the \
              value. Contrast: resource_handle (identity is the \
              resource, not transmissible).",
    anchors: [cast_stdlib::type_properties::serializable::Serializable],
    tags: ["cast_stdlib", "type_properties"],
}
