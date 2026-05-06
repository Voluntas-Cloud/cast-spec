//! `versioned_api_endpoint` — explicit version in API surface.

/// Sentinel for `versioned_api_endpoint`.
pub struct VersionedApiEndpoint;

cast::concept! {
    name: "versioned_api_endpoint",
    summary: "Explicit version in API surface. Path, header, or \
              content-type; lets old and new behaviors coexist.",
    anchors: [cast_stdlib::api::versioned_api_endpoint::VersionedApiEndpoint],
    tags: ["cast_stdlib", "api"],
}
