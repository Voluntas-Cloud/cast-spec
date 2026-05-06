//! `fair_share_scheduler` — CPU distributed according to fairness model.

/// Sentinel for `fair_share_scheduler`.
pub struct FairShareScheduler;

cast::concept! {
    name: "fair_share_scheduler",
    summary: "CPU distributed according to fairness model.",
    anchors: [cast_os_stdlib::scheduling::fair_share_scheduler::FairShareScheduler],
    tags: ["cast_os_stdlib", "scheduling"],
}
