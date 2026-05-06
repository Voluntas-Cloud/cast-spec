//! `versioned_configuration_store` — configuration is stored with history, schema, validation, and rollback.

/// Sentinel for `versioned_configuration_store`.
pub struct VersionedConfigurationStore;

cast::concept! {
    name: "versioned_configuration_store",
    summary: "Configuration is stored with history, schema, \
              validation, and rollback. Composes \
              configuration_schema, schema_versioned_storage, \
              mvcc_generation_log, configuration_snapshot, \
              rollback_operation, policy_as_code, and audit_log. \
              Used for cluster config, feature flags, service \
              settings, policy management, and user preference \
              history.",
    anchors: [cast_stdlib::patterns::versioned_configuration_store::VersionedConfigurationStore],
    tags: ["cast_stdlib", "patterns"],
}
