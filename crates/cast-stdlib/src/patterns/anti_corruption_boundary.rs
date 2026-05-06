//! `anti_corruption_boundary` — translate external models before they touch the domain.

/// Sentinel for `anti_corruption_boundary`.
pub struct AntiCorruptionBoundary;

cast::concept! {
    name: "anti_corruption_boundary",
    summary: "External APIs and models are translated before touching \
              the internal domain. Composes anti_corruption_layer, \
              adapter_pattern, canonical_domain_model, \
              external_id_mapping, schema_mapping, validation_error, \
              and interface_contract. Used for third-party APIs, \
              legacy migrations, finance/email connectors, and \
              identity-provider integrations.",
    anchors: [cast_stdlib::patterns::anti_corruption_boundary::AntiCorruptionBoundary],
    tags: ["cast_stdlib", "patterns"],
}
