//! `automatic_rollback` — revert when health checks fail.

/// Sentinel for `automatic_rollback`.
pub struct AutomaticRollback;

cast::concept! {
    name: "automatic_rollback",
    summary: "Revert when health checks fail. Detection and reversion \
              are part of the deploy system; humans don't get paged \
              to push a button if the platform can do it faster.",
    anchors: [cast_stdlib::deployment::automatic_rollback::AutomaticRollback],
    tags: ["cast_stdlib", "deployment"],
}
