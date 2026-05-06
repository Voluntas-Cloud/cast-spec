//! `typed_interface` — inputs and outputs formally shaped.

/// Sentinel for `typed_interface`.
pub struct TypedInterface;

cast::concept! {
    name: "typed_interface",
    summary: "Inputs and outputs are formally shaped. Compile-time or \
              schema-time enforcement that callers and providers agree \
              on the wire format.",
    anchors: [cast_stdlib::api::typed_interface::TypedInterface],
    tags: ["cast_stdlib", "api"],
}
