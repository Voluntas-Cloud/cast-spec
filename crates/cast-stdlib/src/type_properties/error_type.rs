//! `error_type` — communicates failure across a boundary, carries diagnosis.

/// Sentinel for `error_type`.
pub struct ErrorType;

cast::concept! {
    name: "error_type",
    summary: "Communicates failure across an API boundary. Carries a \
              message and (usually) a category so callers can match \
              and recover. Often shaped as sum_type with a variant \
              per failure mode.",
    anchors: [cast_stdlib::type_properties::error_type::ErrorType],
    tags: ["cast_stdlib", "type_properties"],
}
