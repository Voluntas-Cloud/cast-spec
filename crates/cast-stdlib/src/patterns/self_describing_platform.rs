//! `self_describing_platform` — exposes its capabilities, state, contracts, policies, and diagnostics.

/// Sentinel for `self_describing_platform`.
pub struct SelfDescribingPlatform;

cast::concept! {
    name: "self_describing_platform",
    summary: "The system exposes its capabilities, state, \
              contracts, policies, and diagnostics. Composes \
              capability_discovery, schema_documentation, \
              interface_documentation, debug_endpoint, \
              configuration_schema, service_level_indicator, \
              concept_glossary, and living_documentation. Used for \
              AI-assisted administration, developer onboarding, \
              runtime introspection, service discovery, and \
              automated troubleshooting.",
    anchors: [cast_stdlib::patterns::self_describing_platform::SelfDescribingPlatform],
    tags: ["cast_stdlib", "patterns"],
}
