//! `integration_hub` — external systems connect through adapters, connectors, imports, exports, and webhooks.

/// Sentinel for `integration_hub`.
pub struct IntegrationHub;

cast::concept! {
    name: "integration_hub",
    summary: "External systems connect through adapters, connectors, \
              imports, exports, and webhooks. Composes \
              adapter_integration, connector_pattern, \
              webhook_ingestion, polling_integration, \
              external_id_mapping, canonical_mapping, \
              rate_limit_adapter, and sync_engine. Used for bank \
              imports, email integrations, calendar/task sync, \
              GitHub/GitLab connectors, and government/identity APIs.",
    anchors: [cast_stdlib::patterns::integration_hub::IntegrationHub],
    tags: ["cast_stdlib", "patterns"],
}
