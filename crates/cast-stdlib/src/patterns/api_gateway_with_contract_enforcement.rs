//! `api_gateway_with_contract_enforcement` — single edge enforcing auth, version, limits, schema.

/// Sentinel for `api_gateway_with_contract_enforcement`.
pub struct ApiGatewayWithContractEnforcement;

cast::concept! {
    name: "api_gateway_with_contract_enforcement",
    summary: "All external API access passes through a boundary that \
              enforces auth, versioning, rate limits, and schema. \
              Composes api_gateway_boundary, stable_api_contract, \
              versioned_api_endpoint, authorization_policy, \
              rate_limit_contract, error_contract, schema_validation, \
              and audit_log. Used for public APIs, internal platform \
              APIs, mobile/backend boundaries, and agent-tool API \
              boundaries.",
    anchors: [cast_stdlib::patterns::api_gateway_with_contract_enforcement::ApiGatewayWithContractEnforcement],
    tags: ["cast_stdlib", "patterns"],
}
