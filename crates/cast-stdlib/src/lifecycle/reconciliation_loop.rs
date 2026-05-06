//! `reconciliation_loop` — drive actual toward desired, idempotently.

/// Sentinel for `reconciliation_loop`.
pub struct ReconciliationLoop;

cast::concept! {
    name: "reconciliation_loop",
    summary: "Continuously drive actual state toward desired state. \
              Idempotent, runs on a schedule or on event; the \
              kernel-systemd-Kubernetes pattern.",
    anchors: [cast_stdlib::lifecycle::reconciliation_loop::ReconciliationLoop],
    tags: ["cast_stdlib", "lifecycle"],
}
