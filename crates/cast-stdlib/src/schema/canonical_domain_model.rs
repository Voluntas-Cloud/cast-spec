//! `canonical_domain_model` — one authoritative model for core concepts.

/// Sentinel for `canonical_domain_model`.
pub struct CanonicalDomainModel;

cast::concept! {
    name: "canonical_domain_model",
    summary: "One authoritative model for core concepts. Other layers \
              translate to and from this canonical shape; drift across \
              modules is bounded.",
    anchors: [cast_stdlib::schema::canonical_domain_model::CanonicalDomainModel],
    tags: ["cast_stdlib", "schema"],
}
