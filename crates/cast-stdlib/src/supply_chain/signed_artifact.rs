//! `signed_artifact` — artifact authenticity verified by signature.

/// Sentinel for `signed_artifact`.
pub struct SignedArtifact;

cast::concept! {
    name: "signed_artifact",
    summary: "Artifact authenticity verified by signature. The signing \
              key identifies the producer; consumers verify the \
              signature before installing.",
    anchors: [cast_stdlib::supply_chain::signed_artifact::SignedArtifact],
    tags: ["cast_stdlib", "supply_chain"],
}
