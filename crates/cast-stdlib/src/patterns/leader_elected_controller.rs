//! `leader_elected_controller` — only one controller performs certain actions at a time.

/// Sentinel for `leader_elected_controller`.
pub struct LeaderElectedController;

cast::concept! {
    name: "leader_elected_controller",
    summary: "Only one controller performs certain actions at a \
              time. Composes leader_election, lease_based_lock, \
              fencing_token, heartbeat_signal, distributed_lock, \
              split_brain_prevention, and idempotent_operation. Used \
              for cluster operators, schedulers, singleton \
              reconciliation, backup coordinators, and DNS update \
              controllers.",
    anchors: [cast_stdlib::patterns::leader_elected_controller::LeaderElectedController],
    tags: ["cast_stdlib", "patterns"],
}
