//! API, contract & interface patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod api_gateway_boundary;
pub mod async_job_api;
pub mod backward_compatible_api;
pub mod batch_api;
pub mod bulk_operation;
pub mod callback_signature_verification;
pub mod capability_discovery;
pub mod compatibility_test_suite;
pub mod composable_query;
pub mod error_contract;
pub mod filtering_contract;
pub mod fluent_builder;
pub mod implementation_first_api;
pub mod pagination_contract;
pub mod partial_response;
pub mod rate_limit_contract;
pub mod schema_first_api;
pub mod stable_api_contract;
pub mod typed_interface;
pub mod versioned_api_endpoint;
pub mod webhook_callback;
pub mod wire_format_negotiation;

cast::concept! {
    name: "api",
    summary: "Umbrella for the api stdlib category. API, contract & \
              interface patterns.",
    anchors: [
        crate::api::api_gateway_boundary,
        crate::api::async_job_api,
        crate::api::backward_compatible_api,
        crate::api::batch_api,
        crate::api::bulk_operation,
        crate::api::callback_signature_verification,
        crate::api::capability_discovery,
        crate::api::compatibility_test_suite,
        crate::api::composable_query,
        crate::api::error_contract,
        crate::api::filtering_contract,
        crate::api::fluent_builder,
        crate::api::implementation_first_api,
        crate::api::pagination_contract,
        crate::api::partial_response,
        crate::api::rate_limit_contract,
        crate::api::schema_first_api,
        crate::api::stable_api_contract,
        crate::api::typed_interface,
        crate::api::versioned_api_endpoint,
        crate::api::webhook_callback,
        crate::api::wire_format_negotiation,
    ],
    tags: ["cast_stdlib", "api"],
}

/// Sentinel for the api stdlib group.
pub struct ApiGroup;

cast::rule! {
    rule: "Treat APIs as products — internal APIs too.",
    why:  "'Only we use it' is the traditional last sentence before \
           six teams depend on it. The cost of a bad API compounds; \
           the cost of treating it as a product is small and paid up \
           front.",
    governs: [cast_stdlib::api::ApiGroup],
    tags: ["cast_stdlib", "api"],
}
