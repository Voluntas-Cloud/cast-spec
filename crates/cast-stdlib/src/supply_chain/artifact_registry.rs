//! `artifact_registry` — store built deployable artifacts.

/// Sentinel for `artifact_registry`.
pub struct ArtifactRegistry;

cast::concept! {
    name: "artifact_registry",
    summary: "Store built deployable artifacts. The catalog from \
              which deploy systems pull; access-controlled, \
              content-addressed, and audited.",
    anchors: [cast_stdlib::supply_chain::artifact_registry::ArtifactRegistry],
    tags: ["cast_stdlib", "supply_chain"],
}
