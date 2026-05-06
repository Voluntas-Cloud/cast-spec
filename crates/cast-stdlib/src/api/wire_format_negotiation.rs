//! `wire_format_negotiation` — caller picks the response shape per request.

/// Sentinel for `wire_format_negotiation`.
pub struct WireFormatNegotiation;

cast::concept! {
    name: "wire_format_negotiation",
    summary: "Each request carries a hint (Accept header, `format` \
              parameter, content-type query field) selecting the \
              wire shape of the response — JSON vs YAML vs text vs \
              protobuf — orthogonal to which records are returned. \
              Adding a fourth format means a new renderer, not a new \
              endpoint. Distinct from `partial_response` (which \
              picks *which fields*) and `pagination_contract` (which \
              picks *which page*) — this picks the *encoding*.",
    anchors: [cast_stdlib::api::wire_format_negotiation::WireFormatNegotiation],
    tags: ["cast_stdlib", "api"],
}
