//! `api_gateway_boundary` — shared edge for routing/auth/policy.

/// Sentinel for `api_gateway_boundary`.
pub struct ApiGatewayBoundary;

cast::concept! {
    name: "api_gateway_boundary",
    summary: "Shared edge for routing/auth/policy. Internal services \
              see authenticated, normalized requests; the public-facing \
              concerns live at the edge.",
    anchors: [cast_stdlib::api::api_gateway_boundary::ApiGatewayBoundary],
    tags: ["cast_stdlib", "api"],
}
