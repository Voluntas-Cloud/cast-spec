//! `artifact_manifest` — metadata describing a built artifact.

/// Sentinel for `artifact_manifest`.
pub struct ArtifactManifest;

cast::concept! {
    name: "artifact_manifest",
    summary: "Metadata describing built artifact. Build inputs, \
              version, dependencies, signatures — travels with the \
              artifact so consumers can verify what they have.",
    anchors: [cast_stdlib::supply_chain::artifact_manifest::ArtifactManifest],
    tags: ["cast_stdlib", "supply_chain"],
}
