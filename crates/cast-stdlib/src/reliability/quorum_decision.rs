//! `quorum_decision` — require majority/agreement.

/// Sentinel for `quorum_decision`.
pub struct QuorumDecision;

cast::concept! {
    name: "quorum_decision",
    summary: "Require majority/agreement. A decision is valid only if \
              at least a majority of replicas confirm; protects against \
              single-node failures and prevents split-brain.",
    anchors: [cast_stdlib::reliability::quorum_decision::QuorumDecision],
    tags: ["cast_stdlib", "reliability"],
}
