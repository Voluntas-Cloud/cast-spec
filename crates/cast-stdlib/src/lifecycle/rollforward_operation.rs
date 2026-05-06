//! `rollforward_operation` — fix by moving to a newer valid state.

/// Sentinel for `rollforward_operation`.
pub struct RollforwardOperation;

cast::concept! {
    name: "rollforward_operation",
    summary: "Fix by moving to a newer valid state. Often safer than \
              rollback when downstream effects of the bad state can't \
              be cleanly undone.",
    anchors: [cast_stdlib::lifecycle::rollforward_operation::RollforwardOperation],
    tags: ["cast_stdlib", "lifecycle"],
}
