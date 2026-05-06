//! `release_candidate` — artifact proposed for release.

/// Sentinel for `release_candidate`.
pub struct ReleaseCandidate;

cast::concept! {
    name: "release_candidate",
    summary: "Artifact proposed for release. Tested, signed, \
              attestation-ready; awaiting promotion to general \
              availability or rejection back to development.",
    anchors: [cast_stdlib::supply_chain::release_candidate::ReleaseCandidate],
    tags: ["cast_stdlib", "supply_chain"],
}
