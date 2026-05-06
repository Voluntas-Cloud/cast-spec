//! `resource_request` — declared expected resource use.

/// Sentinel for `resource_request`.
pub struct ResourceRequest;

cast::concept! {
    name: "resource_request",
    summary: "Declared expected resource use. The scheduler reserves \
              this much capacity; if requests are wrong the cluster \
              either oversubscribes (causing eviction) or wastes \
              capacity (causing cost).",
    anchors: [cast_stdlib::resources::resource_request::ResourceRequest],
    tags: ["cast_stdlib", "resources"],
}
