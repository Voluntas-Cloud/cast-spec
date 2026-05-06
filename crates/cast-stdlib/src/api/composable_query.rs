//! `composable_query` — query as orthogonal axes (filter + select + render) the caller composes.

/// Sentinel for `composable_query`.
pub struct ComposableQuery;

cast::concept! {
    name: "composable_query",
    summary: "Expose a query surface as a small set of orthogonal \
              axes the caller composes — typically `select` (which \
              records), `filter` (which subset), `project` (which \
              fields), `render` (which wire shape) — instead of N \
              one-shot endpoints. Adding a new use case becomes a \
              new combination of existing axes, not a new endpoint. \
              The DSL IS the wire format; clients write what they \
              want once and the server doesn't grow per-caller.",
    anchors: [cast_stdlib::api::composable_query::ComposableQuery],
    tags: ["cast_stdlib", "api"],
}
