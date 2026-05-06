//! `migration_gate` — deployment waits for precondition.

/// Sentinel for `migration_gate`.
pub struct MigrationGate;

cast::concept! {
    name: "migration_gate",
    summary: "Deployment waits for precondition. A schema migration \
              has to finish before the new code rolls out; the gate \
              encodes that dependency rather than relying on operator \
              memory.",
    anchors: [cast_stdlib::deployment::migration_gate::MigrationGate],
    tags: ["cast_stdlib", "deployment"],
}
