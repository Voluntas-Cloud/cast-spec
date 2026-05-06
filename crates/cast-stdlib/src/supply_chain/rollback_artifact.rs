//! `rollback_artifact` — known previous deployable version.

/// Sentinel for `rollback_artifact`.
pub struct RollbackArtifact;

cast::concept! {
    name: "rollback_artifact",
    summary: "Known previous deployable version. Identifiable, \
              reachable, and ready to deploy in seconds — the \
              insurance policy for a bad release.",
    anchors: [cast_stdlib::supply_chain::rollback_artifact::RollbackArtifact],
    tags: ["cast_stdlib", "supply_chain"],
}
