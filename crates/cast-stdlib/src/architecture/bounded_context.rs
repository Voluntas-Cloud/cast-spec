//! `bounded_context` — domain language boundary.

/// Sentinel for `bounded_context`.
pub struct BoundedContext;

cast::concept! {
    name: "bounded_context",
    summary: "Domain language boundary. Inside, terms have one meaning; \
              across the boundary they may not match — translation \
              happens at the edge.",
    anchors: [cast_stdlib::architecture::bounded_context::BoundedContext],
    tags: ["cast_stdlib", "architecture"],
}
