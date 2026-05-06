//! Interoperability & integration patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod adapter_integration;
pub mod canonical_mapping;
pub mod change_data_capture;
pub mod conflict_mapping;
pub mod connector_pattern;
pub mod export_pipeline;
pub mod external_id_mapping;
pub mod format_converter;
pub mod import_pipeline;
pub mod integration_health_check;
pub mod polling_integration;
pub mod protocol_bridge;
pub mod rate_limit_adapter;
pub mod sync_engine;
pub mod webhook_ingestion;

cast::concept! {
    name: "integration",
    summary: "Umbrella for the integration stdlib category. \
              Interoperability & integration patterns.",
    anchors: [
        crate::integration::adapter_integration,
        crate::integration::canonical_mapping,
        crate::integration::change_data_capture,
        crate::integration::conflict_mapping,
        crate::integration::connector_pattern,
        crate::integration::export_pipeline,
        crate::integration::external_id_mapping,
        crate::integration::format_converter,
        crate::integration::import_pipeline,
        crate::integration::integration_health_check,
        crate::integration::polling_integration,
        crate::integration::protocol_bridge,
        crate::integration::rate_limit_adapter,
        crate::integration::sync_engine,
        crate::integration::webhook_ingestion,
    ],
    tags: ["cast_stdlib", "integration"],
}

/// Sentinel for the integration stdlib group.
pub struct IntegrationGroup;

cast::rule! {
    rule: "Never let external APIs leak directly into your domain model.",
    why:  "Other people's data models are where elegance goes to be \
           mugged. Translate at the boundary; otherwise every quirk \
           of every external system metastasises through your code.",
    governs: [cast_stdlib::integration::IntegrationGroup],
    tags: ["cast_stdlib", "integration"],
}
