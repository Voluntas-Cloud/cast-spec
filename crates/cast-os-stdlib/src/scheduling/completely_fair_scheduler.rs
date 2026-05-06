//! `completely_fair_scheduler` — Linux-style virtual runtime fairness.

/// Sentinel for `completely_fair_scheduler`.
pub struct CompletelyFairScheduler;

cast::concept! {
    name: "completely_fair_scheduler",
    summary: "Linux-style virtual runtime fairness.",
    anchors: [cast_os_stdlib::scheduling::completely_fair_scheduler::CompletelyFairScheduler],
    tags: ["cast_os_stdlib", "scheduling"],
}
