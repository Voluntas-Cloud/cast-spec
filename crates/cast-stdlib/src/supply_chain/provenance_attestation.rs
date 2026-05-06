//! `provenance_attestation` — record how artifact was built.

/// Sentinel for `provenance_attestation`.
pub struct ProvenanceAttestation;

cast::concept! {
    name: "provenance_attestation",
    summary: "Record how artifact was built. Source commit, builder \
              identity, build steps — signed and verifiable so a \
              consumer can confirm origin without trusting the \
              registry.",
    anchors: [cast_stdlib::supply_chain::provenance_attestation::ProvenanceAttestation],
    tags: ["cast_stdlib", "supply_chain"],
}
